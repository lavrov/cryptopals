pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
    let input_chars: Vec<_> = hex_string.chars().collect();

    input_chars.chunks(2).map(|chunk| {
        let first_byte = chunk[0].to_digit(16).unwrap();
        let second_byte = chunk[1].to_digit(16).unwrap();
        ((first_byte << 4) | second_byte) as u8
    }).collect()
}


pub fn bytes_to_hex(b: &[u8]) -> String {
    use std::fmt::Write;
    let mut out = String::new();
    for byte in b {
        write!(&mut out, "{:x}", byte).expect("Unable to write");
    }
    out
}
