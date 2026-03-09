use std::io::{self, Write};
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use sha1::{Digest, Sha1};
use tungstenite::protocol::Role;
use tungstenite::{Message, WebSocket};

use crate::core::{self, CoreState, LocationCache};
use crate::logger;
use crate::requests::http_request::HttpRequest;
use crate::server::connection_state::ConnectionState;

use super::messages::{ClientMessage, ServerMessage};

pub fn handle(stream: &mut TcpStream, req: &HttpRequest, cache: LocationCache) -> ConnectionState {
    if req.headers.get("Upgrade").map(|v| v.as_str()) != Some("websocket") {
        log_error!("ws", "WS handshake failed: missing Upgrade: websocket header");
        return ConnectionState::Close;
    }

    let key = req.headers
        .get("Sec-WebSocket-Key")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let response = format!(
        "HTTP/1.1 101 Switching Protocols\r\n\
         Upgrade: websocket\r\n\
         Connection: Upgrade\r\n\
         Sec-WebSocket-Accept: {}\r\n\r\n",
        generate_accept_key(&key)
    );

    if let Err(e) = stream.write_all(response.as_bytes()).and_then(|_| stream.flush()) {
        log_error!("ws", "Failed to send WS 101 response: {}", e);
        return ConnectionState::Close;
    }
    log_info!("ws", "WebSocket handshake complete");

    let peer = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let ws_stream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            log_error!("ws", "Failed to clone TcpStream: {}", e);
            return ConnectionState::Close;
        }
    };

    if let Err(e) = ws_stream.set_read_timeout(Some(Duration::from_millis(100))) {
        log_warn!("ws", "Failed to set read timeout: {}", e);
    }

    let mut ws = WebSocket::from_raw_socket(ws_stream, Role::Server, None);
    let log_rx = logger::subscribe();

    send_initial(&mut ws, &cache);
    log_info!("ws", "WebSocket session started for {}", peer);

    run_loop(&mut ws, &log_rx, &cache);

    log_info!("ws", "WebSocket session ended for {}", peer);
    ConnectionState::Close
}

fn run_loop(
    ws: &mut WebSocket<TcpStream>,
    log_rx: &Receiver<logger::LogLine>,
    cache: &LocationCache,
) {
    loop {
        while let Ok(line) = log_rx.try_recv() {
            let msg = ServerMessage::from_log_line(&line);
            if send_msg(ws, &msg).is_err() {
                return;
            }
        }

        match ws.read() {
            Ok(Message::Text(text)) => {
                match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => dispatch(ws, client_msg, cache),
                    Err(e) => log_warn!("ws", "Unparseable client message: {} — {:?}", e, text),
                }
            }
            Ok(Message::Close(_)) => {
                break;
            }
            Ok(Message::Ping(data)) => {
                let _ = ws.send(Message::Pong(data));
            }
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut => {}
            Err(tungstenite::Error::ConnectionClosed)
            | Err(tungstenite::Error::AlreadyClosed) => {
                break;
            }
            Err(e) => {
                log_error!("ws", "WS read error: {}", e);
                break;
            }
        }
    }
}

fn dispatch(ws: &mut WebSocket<TcpStream>, msg: ClientMessage, cache: &LocationCache) {
    log_debug!("ws", "Client message: {:?}", msg);
    match msg {
        ClientMessage::Status => send_status(ws),
        ClientMessage::Connect { location } => match core::connect(location) {
            Ok(()) => send_status(ws),
            Err(e) => send_error(ws, "CommandFailed", &e.to_string()),
        },
        ClientMessage::Disconnect => match core::disconnect() {
            Ok(()) => send_status(ws),
            Err(e) => send_error(ws, "CommandFailed", &e.to_string()),
        },
        ClientMessage::ReconnectWifi => match core::reconnect_wifi() {
            Ok(()) => send_status(ws),
            Err(e) => send_error(ws, "CommandFailed", &e.to_string()),
        },
        ClientMessage::Restart => {
            if let Err(e) = core::restart() {
                send_error(ws, "CommandFailed", &e.to_string());
            }
        }
        ClientMessage::RefreshLocations => {
            cache.refresh_in_background();
            let locs = cache.get();
            let _ = send_msg(ws, &ServerMessage::LocationList { locations: locs });
        }
        ClientMessage::SetLogLevel { level } => match logger::LogLevel::from_str(&level) {
            Some(lvl) => {
                logger::set_level(lvl);
                let _ = send_msg(ws, &ServerMessage::LogLevelChanged { level });
            }
            None => send_error(ws, "InvalidLevel", &format!("Unknown log level: {}", level)),
        },
    }
}

fn send_initial(ws: &mut WebSocket<TcpStream>, cache: &LocationCache) {
    send_status(ws);
    let locs = cache.get();
    let _ = send_msg(ws, &ServerMessage::LocationList { locations: locs });
    let level = format!("{:?}", logger::current_level()).to_lowercase();
    let _ = send_msg(ws, &ServerMessage::LogLevelChanged { level });
}

fn send_status(ws: &mut WebSocket<TcpStream>) {
    let (state, location) = match core::status() {
        Ok(s) => s,
        Err(e) => {
            log_error!("ws", "Failed to get VPN status: {}", e);
            (CoreState::Disconnected, None)
        }
    };
    let _ = send_msg(ws, &ServerMessage::StatusUpdate { state, location });
}

fn send_error(ws: &mut WebSocket<TcpStream>, code: &str, message: &str) {
    let _ = send_msg(
        ws,
        &ServerMessage::Error {
            code:    code.to_string(),
            message: message.to_string(),
        },
    );
}

fn send_msg(ws: &mut WebSocket<TcpStream>, msg: &ServerMessage) -> Result<(), ()> {
    match serde_json::to_string(msg) {
        Ok(json) => ws.send(Message::Text(json)).map_err(|e| {
            log_error!("ws", "WS send error: {}", e);
        }),
        Err(e) => {
            log_error!("ws", "Serialize error: {}", e);
            Err(())
        }
    }
}

fn generate_accept_key(key: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(format!("{}258EAFA5-E914-47DA-95CA-C5AB0DC85B11", key).as_bytes());
    STANDARD.encode(hasher.finalize())
}
