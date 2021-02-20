pub fn decode_frame(content: String) {
    // Decode base64 representation to bytes
    let bytes = base64::decode(content).unwrap();

    // Parse the websocket frame
    let frame: WebSocketFrame = WebSocketFrame::from(bytes);

    let length_padding = "$$$";

    println!(
        "
    1-------10--------20--------30--------40--------50--------60--------70--------80
    ################################################################################
    ## Packet Length: {0: >4} {1}
    ################################################################################
    ## Frame ##
    ##
    {2}
    ################################################################################
    ",
        frame.frame_len,
        length_padding,
        format_short_frame(&frame)
    );

    // for i in 0..bytes.len() {
    //     println!("Byte {0: >2} is {1: >3}: {1:0>8b}", i, bytes[i]);
    // }
}

struct WebSocketFrame {
    frame_len: u8,
    fin_bit: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    opcode: u8,
    mask_bit: bool,
    payload_len: u8,
    masking_key: [u8; 4],
}

impl WebSocketFrame {
    /// Builds a websocket frame from a byte array
    pub fn from(data: Vec<u8>) -> WebSocketFrame {
        WebSocketFrame {
            // Bytes in frame
            frame_len: data.len() as u8,
            // Bit 0 contains fin bit
            fin_bit: get_bit(data[0], 0),
            // Bit 1 contains rsv1
            rsv1: get_bit(data[0], 1),
            // Bit 2 contains rsv2
            rsv2: get_bit(data[0], 2),
            // Bit 3 contains rsv3
            rsv3: get_bit(data[0], 3),
            // Bits 4 - 7 contain the opcode
            opcode: byte(data[0], 0b00001111),
            // Bit 8 contains mask flag
            mask_bit: get_bit(data[1], 0),
            // Bits 9 - 15 contain payload length
            payload_len: byte(data[1], 0b01111111),
            // Next 4 bytes contain masking key
            masking_key: [data[2], data[3], data[4], data[5]]
        }
    }
}

fn byte(byte: u8, mask: u8) -> u8 {
    byte & mask
}

fn format_short_frame(frame: &WebSocketFrame) -> String {
    format!(
        "\
    ## +---------------+---------------+---------------+---------------+
    ## |    Byte  0    |    Byte  1    |    Byte  2    |    Byte  3    |
    ## +---------------+---------------+---------------+---------------+
    ## |0              |    1          |        2      |            3  |
    ## |0 1 2 3 4 5 6 7|8 9 0 1 2 3 4 5|6 7 8 9 0 1 2 3|4 5 6 7 8 9 0 1|
    ## +-+-+-+-+-------+-+-------------+-------------------------------+
    ## |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7} {8}|
    ## |F|R|R|R| opcode|M| Payload len |     Masking-key (part 1)      |
    ## |I|S|S|S|  (4)  |A|     (7)     |              (16)             |
    ## |N|V|V|V|       |S|             |                               |
    ## | |1|2|3|       |K|             |                               |
    ## +-+-+-+-+-------+-+-------------+-------------------------------+
    ## |                               |                               |
    ## |     Masking-key (part 2)      |          Payload Data         |
    ## |              (16)             |                               |
    ## +-------------------------------+-------------------------------+
    ## |                                                               |
    ## |                     Payload Data continued ...                |
    ## +---------------------------------------------------------------+
    ",
        bit_str(frame.fin_bit),
        bit_str(frame.rsv1),
        bit_str(frame.rsv2),
        bit_str(frame.rsv3),
        byte_part_str(frame.opcode, 4),
        bit_str(frame.mask_bit),
        byte_part_str(frame.payload_len, 7),
        byte_part_str(frame.masking_key[0], 8),
        byte_part_str(frame.masking_key[1], 8)
    )
}

fn byte_part_str<'a>(byte: u8, num_bits: u8) -> String {
    let mut result: String = String::from("");
    for i in 8 - num_bits..8 {
        result.push_str(&format!("{} ", bit_str(get_bit(byte, i))));
    }
    result.trim().to_string()
}

fn bit_str<'a>(bit: bool) -> &'a str {
    if bit == true {
        "1"
    } else {
        "0"
    }
}

fn get_bit(byte: u8, bit_position: u8) -> bool {
    match bit_position {
        0 => byte & 0b10000000 != 0,
        1 => byte & 0b01000000 != 0,
        2 => byte & 0b00100000 != 0,
        3 => byte & 0b00010000 != 0,
        4 => byte & 0b00001000 != 0,
        5 => byte & 0b00000100 != 0,
        6 => byte & 0b00000010 != 0,
        7 => byte & 0b00000001 != 0,
        _ => false,
    }
}
