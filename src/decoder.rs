use libwdi::format_qword_table;
use super::websocket_frame::WebSocketFrame;

pub fn decode_frame(content: String) {
    // Decode base64 representation to bytes
    let bytes = base64::decode(content).unwrap();

    // Parse the websocket frame
    let frame: WebSocketFrame = WebSocketFrame::from(&bytes);

    println!(
        "
1-------10--------20--------30--------40--------50--------60--------70--------80
Packet length: {0}

{1}

{2}
    ",
        frame.frame_len,
        format_qword_table(&bytes),
        frame.format_frame()
    );

    // for i in 0..bytes.len() {
    //     println!("Byte {0: >2} is {1: >3}: {1:0>8b}", i, bytes[i]);
    // }
}




#[cfg(test)]
mod tests {
    // gYS8KAcLyE10fw==
}
