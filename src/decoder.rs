use std::str;

const BITS_IN_BYTE: u8 = 8;
const BYTES_IN_DWORD: u8 = 4;

pub fn decode_frame(content: String) {
    // Decode base64 representation to bytes
    let bytes = base64::decode(content).unwrap();

    // Parse the websocket frame
    let frame: WebSocketFrame = WebSocketFrame::from(&bytes);

    let length_padding = "$$$";

    println!(
        "
1-------10--------20--------30--------40--------50--------60--------70--------80
################################################################################
## Packet Length: {0: >4} {1}
################################################################################

{2}

{3}
    ",
        frame.frame_len,
        length_padding,
        format_raw_bytes(&bytes),
        format_short_frame(&frame)
    );

    // for i in 0..bytes.len() {
    //     println!("Byte {0: >2} is {1: >3}: {1:0>8b}", i, bytes[i]);
    // }
}

struct WebSocketFrame<'a> {
    frame_len: u8,
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
            let byte: u8 = data[payload_start_index+i] ^ masking_key[i % NUM_MASK_BYTES];
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
            opcode: byte(data[0], 0b00001111),
            // Bit 8 contains mask flag
            mask_bit: get_bit(data[1], 0),
            // Bits 9 - 15 contain payload length
            payload_len: byte(data[1], 0b01111111),
            // Next 4 bytes contain masking key
            masking_key,
            // Masked payload is from byte 6 to end of frame
            masked_payload: &data[6..data.len()],
            // Unmasked payload
            unmasked_payload,
            payload,
        }
    }
}

fn byte(byte: u8, mask: u8) -> u8 {
    byte & mask
}

fn format_raw_bytes(data: &Vec<u8>) -> String {
    let mut result = format!(
        "
       +--------+--------+--------+--------+--------+--------+--------+--------+
 Bytes | Byte 1 | Byte 2 | Byte 3 | Byte 4 | Byte 5 | Byte 6 | Byte 7 | Byte 8 |
+------+--------+--------+--------+--------+--------+--------+--------+--------+",
    );
    let mut qword_buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    
    let num_qwords = data.len().div_euclid(BITS_IN_BYTE as usize);
    // Append full qwords
    for i in 0..num_qwords {
        let from_byte_ix = i * BITS_IN_BYTE as usize;
        let to_byte_ix = from_byte_ix + BITS_IN_BYTE as usize;
        qword_buf.copy_from_slice(&data[from_byte_ix..to_byte_ix]);
        result.push_str(&format_qword(qword_buf, i + 1));
    }
    // Append final bytes
    let remaining_bytes = data.len().rem_euclid(BITS_IN_BYTE as usize);
    let from_byte_ix = num_qwords * BITS_IN_BYTE as usize;
    let to_byte_ix = from_byte_ix + remaining_bytes as usize;
    result.push_str(&format_partial_qword(&data[from_byte_ix..to_byte_ix], num_qwords + 1, remaining_bytes));
    result
}

fn format_qword(data: [u8; 8], qword_ix: usize) -> String {
    format!(
        "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|{6:0>8b}|{7:0>8b}|
|  {8}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|   ({3:>3})|   ({4:>3})|   ({5:>3})|   ({6:>3})|   ({7:>3})|
+------+--------+--------+--------+--------+--------+--------+--------+--------+",
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], qword_ix
    )
}

fn format_partial_qword(data: &[u8], qword_ix: usize, num_bytes: usize) -> String {
    match num_bytes {
        1 => {
            format!(
                "
|QWORD |{0:0>8b}|
|  {1}   |   ({0:>3})|
+------+--------+",
                data[0], qword_ix
            )
        }
        2 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|
|  {2}   |   ({0:>3})|   ({1:>3})|
+------+--------+--------+",
                data[0], data[1], qword_ix
            )
        }
        3 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|
|  {3}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|
+------+--------+--------+--------+",
                data[0], data[1], data[2], qword_ix
            )
        }
        4 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|{3:0>8b}|
|  {4}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|   ({3:>3})|
+------+--------+--------+--------+--------+",
                data[0], data[1], data[2], data[3], qword_ix
            )
        }
        5 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|
|  {5}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|   ({3:>3})|   ({4:>3})|
+------+--------+--------+--------+--------+--------+",
                data[0], data[1], data[2], data[3], data[4], qword_ix
            )
        }
        6 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|
|  {6}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|   ({3:>3})|   ({4:>3})|   ({5:>3})|
+------+--------+--------+--------+--------+--------+--------+",
                data[0], data[1], data[2], data[3], data[4], data[5], qword_ix
            )
        }
        7 => {
            format!(
                "
|QWORD |{0:0>8b}|{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|{6:0>8b}|
|  {7}   |   ({0:>3})|   ({1:>3})|   ({2:>3})|   ({3:>3})|   ({4:>3})|   ({5:>3})|   ({6:>3})|
+------+--------+--------+--------+--------+--------+--------+--------+",
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], qword_ix
            )
        }
        _ => { String::from("") }
    }
}

fn format_short_frame(frame: &WebSocketFrame) -> String {
    let mut result: String = format!(
        "
               +---------------+---------------+---------------+---------------+
  Frame Data   |    Byte  0    |    Byte  1    |    Byte  2    |    Byte  3    |
   (Masked)    +---------------+---------------+---------------+---------------+
               |0              |    1          |        2      |            3  |
               |0 1 2 3 4 5 6 7|8 9 0 1 2 3 4 5|6 7 8 9 0 1 2 3|4 5 6 7 8 9 0 1|
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
        bit_str(frame.fin_bit),
        bit_str(frame.rsv1),
        bit_str(frame.rsv2),
        bit_str(frame.rsv3),
        byte_str(frame.opcode, 4),
        bit_str(frame.mask_bit),
        byte_str(frame.payload_len, 7),
        byte_str(frame.masking_key[0], 8),
        byte_str(frame.masking_key[1], 8),
        byte_str(frame.masking_key[2], 8),
        byte_str(frame.masking_key[3], 8),
        byte_str(frame.masked_payload[0], 8),
        byte_str(frame.masked_payload[1], 8),
        format!("({})", frame.masked_payload[0]),
        format!("({})", frame.masked_payload[1]),
        byte_str(frame.unmasked_payload[0], 8),
        byte_str(frame.unmasked_payload[1], 8),
        format!("({})", frame.unmasked_payload[0]),
        frame.payload[0],
        format!("({})", frame.unmasked_payload[1]),
        frame.payload[1],
    );

    // Format remaining full dwords
    let remaining_payload_dwords: u8 = (frame.payload_len-2).div_euclid(BYTES_IN_DWORD);
    for i in 0..remaining_payload_dwords {
        let from_byte_ix: usize = ((i * BYTES_IN_DWORD) + 2) as usize;
        let to_byte_ix: usize = from_byte_ix + BYTES_IN_DWORD as usize;
        result.push_str(
            &format_payload_data_dword(
                &frame.masked_payload[from_byte_ix..to_byte_ix],
                &frame.unmasked_payload[from_byte_ix..to_byte_ix],
                &frame.payload[from_byte_ix..to_byte_ix],
                BYTES_IN_DWORD,
                i + 1,
                i + 3
            )
        );
    }

    // Format remaining bytes (formatted as partial dword)
    let remaining_bytes: u8 = (frame.payload_len - 2).rem_euclid(BYTES_IN_DWORD);
    let from_byte_ix: usize = ((remaining_payload_dwords * BYTES_IN_DWORD) + 2) as usize;
    let to_byte_ix: usize = from_byte_ix + remaining_bytes as usize;
    result.push_str(
        &format_payload_data_dword(
            &frame.masked_payload[from_byte_ix..to_byte_ix],
            &frame.unmasked_payload[from_byte_ix..to_byte_ix],
            &frame.payload[from_byte_ix..to_byte_ix],
            remaining_bytes,
            (remaining_payload_dwords * 2) + 2,
            remaining_payload_dwords + 3
        )
    );
    
    result
}

fn format_payload_data_dword(
    masked_bits: &[u8], 
    unmasked_bits: &[u8], 
    data: &[char], 
    num_bytes: u8,
    from_part_number: u8,
    dword_number: u8
) -> String {
    match num_bytes {
        1 => {
            format!(
                "
       | DWORD |{1}|
       |  {0:<2}   | {2:>5}     MSK |
       |       |{3}|
       |       | {4:>5} '{5}' UNM |
       |       |Payload pt {6:<4}|
       +-------+---------------+",
                dword_number,         
                byte_str(masked_bits[0], BITS_IN_BYTE),
                format!("({})", masked_bits[0]),
                byte_str(unmasked_bits[0], BITS_IN_BYTE), 
                format!("({})", unmasked_bits[0]),
                data[0], 
                from_part_number,
            )
        }
        2 => {
            format!(
                "
       | DWORD |{1}|{2}|
       |  {0:<2}   |             MASKED            |
       |       |{3}|{4}|
       |       |     '{5}'    UNMASKED   '{6}'     |
       |       |    Payload Data (part {7:<3})     |
       +-------+-------------------------------+",
                dword_number,         
                byte_str(masked_bits[0], BITS_IN_BYTE), 
                byte_str(masked_bits[1], BITS_IN_BYTE), 
                byte_str(unmasked_bits[0], BITS_IN_BYTE), 
                byte_str(unmasked_bits[1], BITS_IN_BYTE),
                data[0], data[1],
                from_part_number,
            )
        }
        3 => {
            format!(
                "
       | DWORD |{1}|{2}|{3}|                
       |  {0:<2}   |             MASKED            |             MASKED            |
       |       |{4}|{5}|{6}|                
       |       |     '{7}'    UNMASKED   '{8}'     |     '{9}'    UNMASKED           |
       |       |    Payload Data (part {10:<3})    |    Payload Data (part {11:<3})    |
       +-------+-------------------------------+-------------------------------+",
                dword_number,         
                byte_str(masked_bits[0], BITS_IN_BYTE), 
                byte_str(masked_bits[1], BITS_IN_BYTE), 
                byte_str(masked_bits[2], BITS_IN_BYTE), 
                byte_str(unmasked_bits[0], BITS_IN_BYTE), 
                byte_str(unmasked_bits[1], BITS_IN_BYTE), 
                byte_str(unmasked_bits[2], BITS_IN_BYTE),
                data[0], data[1], data[2],
                from_part_number, from_part_number + 1,
            )
        }
        4 => {
            format!(
                "
       | DWORD |{1}|{2}|{3}|{4}|
       |  {0:<2}   |             MASKED            |             MASKED            |
       |       |{5}|{6}|{7}|{8}|
       |       |     '{9}'    UNMASKED   '{10}'     |     '{11}'    UNMASKED   '{12}'     |
       |       |    Payload Data (part {13:<3})    |    Payload Data (part {14:<3})    |
       +-------+-------------------------------+-------------------------------+",
                dword_number,         
                byte_str(masked_bits[0], BITS_IN_BYTE), 
                byte_str(masked_bits[1], BITS_IN_BYTE), 
                byte_str(masked_bits[2], BITS_IN_BYTE), 
                byte_str(masked_bits[3], BITS_IN_BYTE),
                byte_str(unmasked_bits[0], BITS_IN_BYTE), 
                byte_str(unmasked_bits[1], BITS_IN_BYTE), 
                byte_str(unmasked_bits[2], BITS_IN_BYTE), 
                byte_str(unmasked_bits[3], BITS_IN_BYTE),
                data[0], data[1], data[2], data[3],
                from_part_number, from_part_number + 1
            )
        }
        _ => { String::from("ERROR: Can only format between 1 and 4 bytes.") }
    }
}

/// Formats a byte or partial byte.
/// 
/// # Arguments
/// 
/// * `byte` - The byte to format.
/// * `num_bits` - The number of bits to format.
fn byte_str<'a>(byte: u8, num_bits: u8) -> String {
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

#[cfg(test)]
mod tests {
    // gYS8KAcLyE10fw==
}
