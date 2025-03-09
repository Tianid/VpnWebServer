// mod vpn_core;
// mod web_server_core;
// mod request_parser;
// mod requests;
// mod core;
// mod utils;
// mod entities;
// mod entities;
// mod resource_reader;
// mod constatns;
//
// use std::thread;
// use std::fs;
// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};
// use std::sync::{mpsc, Arc, Mutex};
// use crate::vpn_core::{VpnManager, VpnStatus};
// use utils::args_reader::get_server_config;
// use crate::web_server_core::WebServerCore;
//
// fn main() {
//     let config = get_server_config();
//     let config_clone = config.clone();
//     let web_core = WebServerCore::new(config);
//     let listener = TcpListener::bind(format!("{}:{}", config_clone.address, config_clone.port)).expect("Failed to bind to Tcp listener");
//     let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();    thread::spawn(move || {
//         for task in rx {
//             task();
//         }
//     });
//     println!("Running server on {}:{}", config_clone.address, config_clone.port);
//
//
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 let mut web_core_clone = Arc::clone(&web_core);
//                 let tx_clone = tx.clone();
//
//                 tx_clone.send(Box::new(move || {
//                     let mut core = web_core_clone.lock().unwrap();
//                     core.process_request(stream);
//                 })).expect("Ошибка отправки задачи в поток");
//             }
//             Err(e) => { println!("Error: {}", e) }
//         }
//     }
// }
//
// fn old_main() {
//     let config = get_server_config();
//     let (address, port) = (config.address, config.port);
//     println!("Running server on {}:{}", address, port);
//
//     let callback = Arc::new(move |status: VpnStatus| {
//         println!("SEND")
//     });
//
//     let vpn_manager = Arc::new(Mutex::new(VpnManager::new(callback)));
//
//     let listener = TcpListener::bind(format!("{}:{}", address, port)).expect("Failed to bind to Tcp listener");
//
//     let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>(); // Канал для отправки задач в поток
//     thread::spawn(move || {
//         for task in rx {
//             task()
//         }
//     });
//
//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 let vpn_manager = Arc::clone(&vpn_manager);
//
//                 tx.send(Box::new(move || {
//                     handle_client_requests(stream, vpn_manager)
//                 })).unwrap();
//             }
//             Err(e) => { println!("Error: {}", e) }
//         }
//     }
// }
//
//
//
//
// fn handle_client_requests(mut stream: TcpStream, vpn_manager: Arc<Mutex<VpnManager>>) {
//     let mut buffer = [0; 1024];
//     let bytes_read = stream.read(&mut buffer).unwrap_or(0);
//
//     if bytes_read == 0 { return }
//
//     let request = String::from_utf8_lossy(&buffer[..bytes_read]);
//
//     if request.starts_with("GET / HTTP/1.1") {
//         println!("Receive GET request");
//         send_response(&mut stream, "index.html", 200);
//     } else if request.starts_with("GET /status HTTP/1.1") {
//         println!("Receive GET /status request");
//         send_status(&mut stream, vpn_manager.clone());
//     } else if request.starts_with("POST /start HTTP/1.1") {
//         println!("Receive POST /start request");
//         vpn_manager.lock().unwrap().start();
//         send_response_text(&mut stream, "Ok", 200);
//     } else if request.starts_with("POST /stop HTTP/1.1") {
//         println!("Receive POST /stop request");
//         vpn_manager.lock().unwrap().stop();
//         send_response_text(&mut stream, "Ok", 200);
//     } else {
//         println!("Restricted request {}", request.to_string());
//         send_response_text(&mut stream, "404 Not Found", 404);
//     }
// }
//
// fn send_response(stream: &mut TcpStream, filename: &str, status_code: u16) {
//     let file_content = fs::read_to_string(format!("web_pages/{}", filename));
//     let response = match file_content {
//         Ok(content) => format!(
//             "HTTP/1.1 {} OK\r\nContent-Type: text/html\r\n\r\n{}",
//             status_code, content
//         ),
//         Err(_) => "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nФайл не найден!".to_string(),
//     };
//     stream.write_all(response.as_bytes()).unwrap();
// }
//
// fn send_response_text(stream: &mut TcpStream, text: &str, status_code: u16) {
//     let response = format!(
//         "HTTP/1.1 {} OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
//         status_code,
//         text.len(),
//         text
//     );
//     stream.write_all(response.as_bytes()).unwrap();
// }
//
//
// fn send_status(stream: &mut TcpStream, manager: Arc<Mutex<VpnManager>>) {
//     let status = match manager.lock().unwrap().get_status() {
//         VpnStatus::Disconnecting => "Disconnecting",
//         VpnStatus::Disconnected => "Disconnected",
//         VpnStatus::Connecting => "Connecting",
//         VpnStatus::Connected => "Connected",
//         VpnStatus::Reconnecting => "Reconnecting",
//     };
//     send_response_text(stream, status, 200);
// }

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::core::http_request_processor::process;
use crate::requests::http_request::HttpRequest;

mod responses;
mod core;
mod requests;
mod generated;
mod utils;
mod entities;
mod web_server;
mod logger;

fn main() {
    web_server::start(web_server::web_server_configuration::WebServerConfiguration::new("127.0.0.1".as_ref(), "9000".as_ref()));
    // let listener = TcpListener::bind("127.0.0.1:9000").expect("Failed to bind port");

    // println!("WebSocket server started at ws://127.0.0.1:9000/ws");



    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             std::thread::spawn(move || {
    //                 handle_client(&mut stream);
    //             });
    //         }
    //         Err(e) => eprintln!("Connection failed: {}", e),
    //     }
    // }
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];

    if let Ok(size) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer[..size]);

        match HttpRequest::new(request.as_ref()) {
            Some(http_request) => {
                println!("[INFO] Received http request: {:?}", http_request);
                process(stream, &http_request);
            }
            None => { println!("[INFO] Received no http request"); }
        }
    }
}
