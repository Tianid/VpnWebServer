pub fn encode_message(message: &str) -> Vec<u8> {
    let mut frame = vec![0x81];
    let message_bytes = message.as_bytes();
    let length = message_bytes.len();

    if length < 126 {
        frame.push(length as u8);
    } else if length < 65536 {
        frame.push(126);
        frame.push(((length >> 8) & 0xFF) as u8);
        frame.push((length & 0xFF) as u8);
    } else {
        frame.push(127);
        for i in (0..8).rev() {
            frame.push(((length >> (i * 8)) & 0xFF) as u8);
        }
    }

    frame.extend_from_slice(message_bytes);
    frame
}

pub fn decode_message(buffer: &[u8]) -> String {
    let payload_length = (buffer[1] & 0x7F) as usize;
    let masking_key = &buffer[2..6];
    let masked_data = &buffer[6..(6 + payload_length)];

    let mut decoded_data = vec![0; payload_length];
    for i in 0..payload_length {
        decoded_data[i] = masked_data[i] ^ masking_key[i % 4];
    }

    String::from_utf8_lossy(&decoded_data).to_string()
}
