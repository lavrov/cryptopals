pub fn fixed_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

#[test]
fn example_hex_xor() {
    use convert::*;
    let a = hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let b = hex_to_bytes("686974207468652062756c6c277320657965");
    let result = bytes_to_hex(&fixed_xor(&a, &b));
    assert_eq!(result, "746865206b696420646f6e277420706c6179");
}
