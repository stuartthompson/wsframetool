const BITS_IN_BYTE: u8 = 8;

pub fn format_raw_bytes(data: &Vec<u8>) -> String {
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
    result.push_str(&format_partial_qword(
        &data[from_byte_ix..to_byte_ix],
        num_qwords + 1,
        remaining_bytes,
    ));
    result
}

pub fn format_qword(data: [u8; 8], qword_ix: usize) -> String {
    format!(
        "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|{6:0>8b}|{7:0>8b}|{8:0>8b}|
|{0:^6}|{9:>8}|{10:>8}|{11:>8}|{12:>8}|{13:>8}|{14:>8}|{15:>8}|{16:>8}|
+------+--------+--------+--------+--------+--------+--------+--------+--------+",
        qword_ix,
        data[0],
        data[1],
        data[2],
        data[3],
        data[4],
        data[5],
        data[6],
        data[7],
        format!("({})", data[0]),
        format!("({})", data[1]),
        format!("({})", data[2]),
        format!("({})", data[3]),
        format!("({})", data[4]),
        format!("({})", data[5]),
        format!("({})", data[6]),
        format!("({})", data[7]),
    )
}

pub fn format_partial_qword(data: &[u8], qword_ix: usize, num_bytes: usize) -> String {
    match num_bytes {
        1 => {
            format!("
|QWORD |{1:0>8b}|
|{0:^6}|{2:>8}|
+------+--------+",
                qword_ix,
                data[0],
                format!("({})", data[0])
            )
        }
        2 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|
|{0:^6}|{3:>8}|{4:>8}|
+------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                format!("({})", data[0]),
                format!("({})", data[1]),
            )
        }
        3 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|
|{0:^6}|{4:>8}|{5:>8}|{6:>8}|
+------+--------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                data[2],
                format!("({})", data[0]),
                format!("({})", data[1]),
                format!("({})", data[2]),
            )
        }
        4 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|
|{0:^6}|{5:>8}|{6:>8}|{7:>8}|{8:>8}|
+------+--------+--------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                data[2],
                data[3],
                format!("({})", data[0]),
                format!("({})", data[1]),
                format!("({})", data[2]),
                format!("({})", data[3]),
            )
        }
        5 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|
|{0:^6}|{6:>8}|{7:>8}|{8:>8}|{9:>8}|{10:>8}|
+------+--------+--------+--------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                data[2],
                data[3],
                data[4],
                format!("({})", data[0]),
                format!("({})", data[1]),
                format!("({})", data[2]),
                format!("({})", data[3]),
                format!("({})", data[4]),
            )
        }
        6 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|{6:0>8b}|
|{0:^6}|{7:>8}|{8:>8}|{9:>8}|{10:>8}|{11:>8}|{12:>8}|
+------+--------+--------+--------+--------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                data[2],
                data[3],
                data[4],
                data[5],
                format!("({})", data[0]),
                format!("({})", data[1]),
                format!("({})", data[2]),
                format!("({})", data[3]),
                format!("({})", data[4]),
                format!("({})", data[5]),
            )
        }
        7 => {
            format!(
                "
|QWORD |{1:0>8b}|{2:0>8b}|{3:0>8b}|{4:0>8b}|{5:0>8b}|{6:0>8b}|{7:0>8b}|
|{0:^6}|{8:>8}|{9:>8}|{10:>8}|{11:>8}|{12:>8}|{13:>8}|{14:>8}|
+------+--------+--------+--------+--------+--------+--------+--------+",
                qword_ix,
                data[0],
                data[1],
                data[2],
                data[3],
                data[4],
                data[5],
                data[6],
                format!("({})", data[0]),
                format!("({})", data[1]),
                format!("({})", data[2]),
                format!("({})", data[3]),
                format!("({})", data[4]),
                format!("({})", data[5]),
                format!("({})", data[6]),
            )
        }
        _ => String::from("ERROR: Can only format up to 7 bytes."),
    }
}
