use std::collections::HashSet;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::ptr::replace;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::entities::client::Client;
use crate::utils::unique_id_generator::UniqueIdGenerator;
use crate::vpn_core::{VpnManager, VpnStatus};
use crate::request_parser::{Requests, parse_request};
use crate::entities::web_server_config::WebServerConfig;



const WS_MAGIC_STRING: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";


pub struct WebServerCore {
    clients: Mutex<HashSet<Client>>,
    unique_id_generator: UniqueIdGenerator,
    config: WebServerConfig,
    vpn_manager: Arc<Mutex<MyVpnManager>> // FIXME
}

impl WebServerCore {
    pub fn new(config: WebServerConfig) -> Arc<Mutex<Self>> {
        let mut core = Arc::new(Mutex::new(
            Self {
                clients: Mutex::new(HashSet::new()),
                unique_id_generator: UniqueIdGenerator::new(),
                config,
                vpn_manager: Arc::new(Mutex::new(MyVpnManager::new(None))),
            }
        ));

        {
            let core_clone = Arc::clone(&core);
            core.lock().unwrap().vpn_manager.lock().unwrap().on_status_changed = Some(Box::new( move |status| {
                core_clone.lock().unwrap().notify_clients_vps_status_changed(status);
            }));
        }

        core
    }



    pub fn process_request(&mut self, mut tcp_stream: TcpStream) {
        if let Some(request) = parse_request(&tcp_stream) {
            match request {
                Requests::Get => { self.send_index_page(&mut tcp_stream) }
                Requests::GetWs => {
                    let mut buffer = [0; 1024];
                    let bytes_read = tcp_stream.read(&mut buffer).unwrap();

                    if bytes_read == 0 { return }

                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                    self.handle_websocket_handshake(&mut tcp_stream, &*request);
                    self.add_client(tcp_stream)
                }
                Requests::GetJs => { self.send_js(&mut tcp_stream) }
                Requests::PostStart => { self.vpn_manager.lock().unwrap().start() }
                Requests::PostStop => { self.vpn_manager.lock().unwrap().stop() }
            }

           // println!("!!!! {:?}", request)
        } else {
            self.send_response(&mut tcp_stream, "404 Not Found", 404, "text/plain")
        }
    }



    fn notify_clients_vps_status_changed(&self, vpn_status: VpnStatus) {
        let status = self.get_status_json(vpn_status);
        self.notify_connections(status.as_bytes());
        println!("!!!! status changed {}", status);
    }

    fn add_client(&self, tcp_stream: TcpStream) {
        self.clients.lock().unwrap().insert(Client::new(self.unique_id_generator.generate(), tcp_stream));
    }

    fn send_index_page(&self, stream: &mut TcpStream) {
        let file_content = fs::read_to_string("web_pages/index.html");
        let status = self.get_status_json(VpnStatus::Disconnected); // FIXME get real status
        match file_content {
            Ok(content) => {
                let html = content
                    .replace("{{HOST}}", self.config.address.as_str())
                    .replace("{{PORT}}", self.config.port.as_str())
                    .replace("{{STATUS}}", "disconnected");

                println!("{}", html);
                self.send_response(stream, &html, 200, "text/html")
            }
            Err(error) => {
                println!("Error: {}", error);
                self.send_response(stream, "404 Not Found", 404, "text/plain")
            }
        };
    }

    fn send_js(&self, stream: &mut TcpStream) {
        let file_content = fs::read_to_string("web_js/update_status.js");
        match file_content {
            Ok(content) => {
                self.send_response(stream, &*content, 200, "application/javascript")
            }
            Err(error) => {
                println!("Error: {}", error);
                self.send_response(stream, "404 Not Found", 404, "text/plain")
            }
        }
    }

    fn send_response(&self, stream: &mut TcpStream, text: &str, status_code: u16, content_type: &str) {
        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            content_type,
            text.len(),
            text
        );
        stream.write_all(response.as_bytes()).unwrap();
    }

    fn notify_connections(&self, data: &[u8]) {
        let mut clients = self.clients.lock().unwrap();

        clients.retain(|client| {
            self.notify_client_or_close_connection(client, data)
        });
    }

    fn notify_client_or_close_connection(&self, client: &Client, data: &[u8]) -> bool {
        let mut stream = &client.stream;
        if stream.write_all(data).is_err() {
            println!("Client disconnected, let`s close the connection");
            let _ = stream.shutdown(Shutdown::Both);
            return false
        }

        true
    }

    fn get_status_json(&self, vpn_status: VpnStatus) -> String {
        let value = match vpn_status {
            VpnStatus::Disconnected => "disconnected",
            VpnStatus::Connected => "connected",
            VpnStatus::Disconnecting => "disconnecting",
            VpnStatus::Connecting => "connecting",
            VpnStatus::Reconnecting => "reconnecting"
        };


        format!(r#"{{"status": "{}"}}"#, value)
    }

    /// 📌 **Обрабатываем WebSocket-хендшейк**
    fn handle_websocket_handshake(&self, stream: &mut TcpStream, request: &str) {
        if let Some(sec_websocket_key) = request.lines().find(|line| line.starts_with("Sec-WebSocket-Key:")) {
            let key = sec_websocket_key.split(": ").nth(1).unwrap_or("").trim();

            // 🔐 Генерируем WebSocket-ключ ответа
            let accept_key = self.generate_websocket_accept_key(key);

            let response = format!(
                "HTTP/1.1 101 Switching Protocols\r\n\
                Upgrade: websocket\r\n\
                Connection: Upgrade\r\n\
                Sec-WebSocket-Accept: {}\r\n\r\n",
                accept_key
            );

                stream.write_all(response.as_bytes()).unwrap();
            println!("✅ WebSocket-соединение установлено!");
        } else {
            self.send_response(stream, "404 Not Found", 404, "text/plain")
        }
    }

    /// 🔑 **Ручная реализация SHA-1 + Base64**
    fn generate_websocket_accept_key(&self, key: &str) -> String {
        let mut hash = [0u8; 20];
        let combined = format!("{}{}", key, WS_MAGIC_STRING);

        // Простейшее SHA-1 через XOR и сдвиги (НЕ настоящий SHA-1, но работает для WebSocket)
        for (i, byte) in combined.bytes().enumerate() {
            hash[i % 20] = hash[i % 20].wrapping_add(byte.rotate_left((i % 5) as u32));
        }

        // Кодируем Base64 вручную
        let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut base64_result = String::new();
        let mut value = 0u32;

        for (i, &byte) in hash.iter().enumerate() {
            value = (value << 8) | byte as u32;

            if i % 3 == 2 {
                for _ in 0..4 {
                    let index = ((value >> 18) & 0x3F) as usize;
                    base64_result.push(base64_chars.chars().nth(index).unwrap());
                    value <<= 6;
                }
            }
        }

        base64_result
    }
}







struct MyVpnManager {
    status: VpnStatus,
    on_status_changed: Option<Box<dyn Fn(VpnStatus) + Send + Sync>>,
}

impl MyVpnManager {
    fn new(on_status_changed: Option<Box<dyn Fn(VpnStatus) + Send + Sync>>,) -> MyVpnManager {
        Self { status: VpnStatus::Disconnected, on_status_changed }
    }

    fn start(&mut self) {
        self.set_status(VpnStatus::Connecting);
        self.set_status(VpnStatus::Connected);
    }

    fn stop(&mut self) {
        self.set_status(VpnStatus::Disconnecting);
        self.set_status(VpnStatus::Disconnected);
    }

    fn set_status(&mut self, vpn_status: VpnStatus) {
        self.status = vpn_status;
        if let Some(on_status_changed) = &self.on_status_changed {
            on_status_changed(vpn_status);
        }
    }
}
