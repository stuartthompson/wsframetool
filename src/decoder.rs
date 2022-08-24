use bitformat::{QwordTable, WebSocketFrame};

pub fn decode_frame(content: String) {
    // Decode base64 representation to bytes
    let bytes = base64::decode(content).unwrap();

    // Parse the websocket frame
    let raw_byte_table: QwordTable = QwordTable::from_bytes(&bytes);
    let frame: WebSocketFrame = WebSocketFrame::from_bytes(&bytes);

    println!(
        "
1-------10--------20--------30--------40--------50--------60--------70--------80
Packet length: {0}

{1}

{2}
    ",
        frame.frame_len,
        raw_byte_table.format(),
        frame.format()
    );
}

#[cfg(test)]
mod tests {
    
}
