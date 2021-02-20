pub fn decode_frame(content: String) {
    // Decode base64 representation to bytes
    let decoded = base64::decode(content).unwrap();

    // Packet length
    let packet_length = decoded.len();

    print_frame(packet_length, decoded);
}

fn print_frame(length: usize, bytes: Vec<u8>) {
    let length_padding = "$$$";

    println!(
        "
    1-------10--------20--------30--------40--------50--------60--------70--------80
    ################################################################################
    ## Packet Length: {0: >4} {1}
    ################################################################################
    ## Frame ##
    ## {2}
    ################################################################################
    ",
        length, length_padding, format_short_frame(&bytes)
    );

    for i in 0..bytes.len() {
        println!("Byte {0: >2} is {1: >3}: {1:0>8b}", i, bytes[i]);
    }
}

fn format_short_frame(bytes: &Vec<u8>) -> String {
    format!(
        "
     0                   1                   2                   3
     0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    +-+-+-+-+-------+-+-------------+-------------------------------+
    | | | | |       | |             |                               |
    |F|R|R|R| opcode|M| Payload len |     Masking-key (part 1)      |
    |I|S|S|S|  (4)  |A|     (7)     |              (16)             |
    |N|V|V|V|       |S|             |                               |
    | |1|2|3|       |K|             |                               |
    +-+-+-+-+-------+-+-------------+-------------------------------+
    |                               |                               |
    |     Masking-key (part 2)      |          Payload Data         |
    |              (16)             |                               |
    +-------------------------------+-------------------------------+
    |                                                               |
    |                     Payload Data continued ...                |
    +---------------------------------------------------------------+
    "
    )
}
