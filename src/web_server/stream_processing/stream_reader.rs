use std::io::Read;
use std::net::TcpStream;
use crate::web_server::connection_state::ConnectionState;
use crate::web_server::stream_processing::stream_data_processor::StreamDataProcessor;
use crate::logger;

pub fn read_from_stream<T: StreamDataProcessor>(
    buffer_size: usize,
    stream: &mut TcpStream,
    processor: &T,
) -> ConnectionState {
    let mut buffer = vec![0; buffer_size];

    match stream.read(&mut buffer) {
        Ok(0)       => {
            logger::info("Client close connection");
            println!("!!!! {}", String::from_utf8_lossy(&buffer));
            ConnectionState::Close
        }
        Ok(size)    => processor.process(stream, size, &buffer),
        Err(error)  => {
            logger::error(format!("Failed to read data from stream, error={}", error).as_str());
            ConnectionState::Close
        }
    }
}
