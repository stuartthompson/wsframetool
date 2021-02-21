const BITS_IN_BYTE: u8 = 8;
const BYTES_IN_DWORD: u8 = 4;

pub struct WebSocketFrame<'a> {
    pub frame_len: u8,
    fin_bit: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    opcode: u8,
    mask_bit: bool,
    payload_len: u8,
    masking_key: [u8; 4],
    masked_payload: &'a [u8],
    unmasked_payload: Vec<u8>,
    payload: Vec<char>,
}

impl<'a> WebSocketFrame<'a> {
    /// Builds a websocket frame from a byte array
    pub fn from(data: &Vec<u8>) -> WebSocketFrame {
        const NUM_MASK_BYTES: usize = 4;

        // Get frame length
        let frame_length: usize = data.len();

        // TODO: Handle larger payloads and unmasked payloads
        let payload_start_index = 6;

        let num_payload_bytes: usize = frame_length - payload_start_index;

        // Get mask
        let masking_key: [u8; 4] = [data[2], data[3], data[4], data[5]];

        // Unmask and parse payload data
        let mut unmasked_payload: Vec<u8> = Vec::new();
        let mut payload: Vec<char> = Vec::new();
        for i in 0..num_payload_bytes {
            let byte: u8 = data[payload_start_index + i] ^ masking_key[i % NUM_MASK_BYTES];
            unmasked_payload.push(byte); // 32 mask bits are used repeatedly
                                         //payload.push(byte as char);
            payload.push(byte as char);
        }

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
            opcode: get_bits_from_byte(data[0], 0b00001111),
            // Bit 8 contains mask flag
            mask_bit: get_bit(data[1], 0),
            // Bits 9 - 15 contain payload length
            payload_len: get_bits_from_byte(data[1], 0b01111111),
            // Next 4 bytes contain masking key
            masking_key,
            // Masked payload is from byte 6 to end of frame
            masked_payload: &data[6..data.len()],
            // Unmasked payload
            unmasked_payload,
            payload,
        }
    }

    /// Formats the websocket frame.
    /// 
    /// # Arguments
    /// 
    /// * `self` - 
    pub fn format_frame(self: &WebSocketFrame<'a>) -> String {
        let mut result = format_frame_header(true, true);
    
        result.push_str(&format!(
            "
       +-------+-+-+-+-+-------+-+-------------+-------------------------------+
       | DWORD |{0}|{1}|{2}|{3}|{4}|{5}|{6}|{7}|{8}|
       |   1   |F|R|R|R|       |M|             |                               |
       |       |I|S|S|S|op code|A| Payload len |     Masking-key (part 1)      |
       |       |N|V|V|V| (4 b) |S|  (7 bits)   |           (16 bits)           |
       |       | |1|2|3|       |K|             |                               |
       +-------+-+-+-+-+-------+-+-------------+-------------------------------+
       | DWORD |{9}|{10}|{11}|{12}|
       |   2   |                               | {13:>5}      MASKED  {14:>5}      |
       |       |     Masking-key (part 2)      |{15}|{16}|
       |       |           (16 bits)           | {17:>5} '{18}' UNMASKED {19:>5} '{20}'  |
       |       |                               |     Payload Data (part 1)     |       
       +-------+-------------------------------+-------------------------------+",
            bit_str(self.fin_bit),
            bit_str(self.rsv1),
            bit_str(self.rsv2),
            bit_str(self.rsv3),
            byte_str(self.opcode, 4),
            bit_str(self.mask_bit),
            byte_str(self.payload_len, 7),
            byte_str(self.masking_key[0], 8),
            byte_str(self.masking_key[1], 8),
            byte_str(self.masking_key[2], 8),
            byte_str(self.masking_key[3], 8),
            byte_str(self.masked_payload[0], 8),
            byte_str(self.masked_payload[1], 8),
            format!("({})", self.masked_payload[0]),
            format!("({})", self.masked_payload[1]),
            byte_str(self.unmasked_payload[0], 8),
            byte_str(self.unmasked_payload[1], 8),
            format!("({})", self.unmasked_payload[0]),
            self.payload[0],
            format!("({})", self.unmasked_payload[1]),
            self.payload[1],
        ));
    
        // Format remaining full dwords
        let remaining_payload_dwords: u8 = (self.payload_len - 2).div_euclid(BYTES_IN_DWORD);
        for i in 0..remaining_payload_dwords {
            let from_byte_ix: usize = ((i * BYTES_IN_DWORD) + 2) as usize;
            let to_byte_ix: usize = from_byte_ix + BYTES_IN_DWORD as usize;
            result.push_str(&format_payload_dword_row(
                &self.masked_payload[from_byte_ix..to_byte_ix],
                &self.unmasked_payload[from_byte_ix..to_byte_ix],
                &self.payload[from_byte_ix..to_byte_ix],
                BYTES_IN_DWORD as usize,
                i + 2,
                i + 3,
            ));
        }
    
        // Format remaining bytes (formatted as partial dword)
        let remaining_bytes: u8 = (self.payload_len - 2).rem_euclid(BYTES_IN_DWORD);
        if remaining_bytes > 0 {
            let from_byte_ix: usize = ((remaining_payload_dwords * BYTES_IN_DWORD) + 2) as usize;
            let to_byte_ix: usize = from_byte_ix + remaining_bytes as usize;
            result.push_str(&format_payload_dword_row(
                &self.masked_payload[from_byte_ix..to_byte_ix],
                &self.unmasked_payload[from_byte_ix..to_byte_ix],
                &self.payload[from_byte_ix..to_byte_ix],
                remaining_bytes as usize,
                (remaining_payload_dwords * 2) + 2,
                remaining_payload_dwords + 3,
            ));
        }
        result
    }
}

fn get_bits_from_byte(byte: u8, mask: u8) -> u8 {
    byte & mask
}

fn format_frame_header(masked: bool, short: bool) -> String {
    format!(
        "
               +---------------+---------------+---------------+---------------+
  Frame Data   |    Byte  0    |    Byte  1    |    Byte  2    |    Byte  3    |
  {0:^10}   +---------------+---------------+---------------+---------------+
  {1:^10}   |0              |    1          |        2      |            3  |
               |0 1 2 3 4 5 6 7|8 9 0 1 2 3 4 5|6 7 8 9 0 1 2 3|4 5 6 7 8 9 0 1|",
        if masked { "(Masked)" } else { "(Unmasked)" },
        if short { "(Short)" } else { "(Long) " }
    )
}

/// Formats a dword table row displaying part of a websocket frame payload.
fn format_payload_dword_row(
    masked_bits: &[u8],
    unmasked_bits: &[u8],
    data: &[char],
    num_bytes: usize,
    from_part_number: u8,
    dword_number: u8,
) -> String {
    let mut result: String = String::from("");

    // Format masked bits
    result.push_str("\n       | DWORD |");
    result.push_str(
        &(0..num_bytes)
            .map(|i| format!("{}|", byte_str(masked_bits[i], BITS_IN_BYTE)))
            .collect::<String>(),
    );

    // Format masked char previews
    result.push_str(&format!("\n       | {:^5} |", dword_number));
    match num_bytes {
        1 => result.push_str(&format!(" {:>5}     MSK |", format!("({})", masked_bits[0]))),
        2 => result.push_str(&format!(
            " {0:>5}      MASKED  {1:>5}      |",
            format!("({})", masked_bits[0]),
            format!("({})", masked_bits[1])
        )),
        3 => result.push_str(&format!(
            " {0:>5}      MASKED  {1:>5}      | {2:>5}     MSK |",
            format!("({})", masked_bits[0]),
            format!("({})", masked_bits[1]),
            format!("({})", masked_bits[2])
        )),
        4 => result.push_str(&format!(
            " {0:>5}      MASKED  {1:>5}      | {2:>5}      MASKED  {3:>5}      |",
            format!("({})", masked_bits[0]),
            format!("({})", masked_bits[1]),
            format!("({})", masked_bits[2]),
            format!("({})", masked_bits[3])
        )),
        _ => {}
    }

    // Format unmasked bits
    result.push_str("\n       |       |");
    result.push_str(
        &(0..num_bytes)
            .map(|i| format!("{}|", byte_str(unmasked_bits[i], BITS_IN_BYTE)))
            .collect::<String>(),
    );

    // Format unmasked char previews
    result.push_str("\n       |       |");
    match num_bytes {
        1 => result.push_str(&format!(
            " {0:>5} '{1}' UNM |",
            format!("({})", unmasked_bits[0]),
            data[0]
        )),
        2 => result.push_str(&format!(
            " {0:>5} '{1}' UNMASKED {2:>5} '{3}'  |",
            format!("({})", unmasked_bits[0]),
            data[0],
            format!("({})", unmasked_bits[1]),
            data[1]
        )),
        3 => result.push_str(&format!(
            " {0:>5} '{1}' UNMASKED {2:>5} '{3}'  | {4:>5} '{5}' UNM |",
            format!("({})", unmasked_bits[0]),
            data[0],
            format!("({})", unmasked_bits[1]),
            data[1],
            format!("({})", unmasked_bits[2]),
            data[2],
        )),
        4 => result.push_str(&format!(
            " {0:>5} '{1}' UNMASKED {2:>5} '{3}'  | {4:>5} '{5}' UNMASKED {6:>5} '{7}'  |",
            format!("({})", unmasked_bits[0]),
            data[0],
            format!("({})", unmasked_bits[1]),
            data[1],
            format!("({})", unmasked_bits[2]),
            data[2],
            format!("({})", unmasked_bits[3]),
            data[3],
        )),
        _ => {}
    }

    // Format payload part text
    result.push_str("\n       |       |");
    match num_bytes {
        1 => result.push_str(&format!("{:^15}|", format!("Payload pt {}", from_part_number))),
        2 => result.push_str(&format!(
            "{:^31}|",
            format!("Payload Data (part {})", from_part_number)
        )),
        3 => result.push_str(&format!(
            "{0:^31}|{1:^15}|",
            format!("Payload Data (part {})", from_part_number),
            format!("Payload pt {}", from_part_number + 1)
        )),
        4 => result.push_str(&format!(
            "{0:^31}|{1:^31}|",
            format!("Payload Data (part {})", from_part_number),
            format!("Payload Data (part {})", from_part_number + 1)
        )),
        _ => {},
    }

    // Format bottom border
    result.push_str("\n       +-------+");
    result.push_str(
        &(0..num_bytes)
            .map(|_| "---------------+")
            .collect::<String>(),
    );
    result.push_str("\n");

    result
}

/// Formats a byte or partial byte.
///
/// # Arguments
///
/// * `byte` - The byte to format.
/// * `num_bits` - The number of bits to format.
fn byte_str<'a>(byte: u8, num_bits: u8) -> String {
    let mut result: String = String::from("");
    result.push_str(
        &(8 - num_bits..8)
            .map(|i| format!("{} ", bit_str(get_bit(byte, i))))
            .collect::<String>()
        );
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