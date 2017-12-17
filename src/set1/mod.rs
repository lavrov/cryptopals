const BASE_64: [u8; 64] = [
    b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',
    b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',
    b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',
    b'Y',b'Z',b'a',b'b',b'c',b'd',b'e',b'f',
    b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',
    b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',
    b'w',b'x',b'y',b'z',b'0',b'1',b'2',b'3',
    b'4',b'5',b'6',b'7',b'8',b'9',b'+',b'/'
];

pub fn to_base64(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut iter = input.into_iter();
    loop {
        let output_ref = &mut output;
        match iter.next() {
            None => break,
            Some(b1) => {
                let mut index = b1 >> 2;
                push(output_ref, index);
                index = (b1 & 0b11) << 4;
                match iter.next() {
                    None =>
                        push(output_ref, index),
                    Some(b2) => {
                        index = index | (b2 >> 4) ;
                        push(output_ref, index);
                        index = (b2 & 0b1111) << 2;
                        match iter.next() {
                            None =>
                                push(output_ref, index),
                            Some(b3) => {
                                index = index | (b3 >> 6);
                                push(output_ref, index);
                                index = b3 & 0b111111;
                                push(output_ref, index);
                            }
                        }
                    }
                }
            }
        }
    };
    output
}

#[inline]
fn push(vec: &mut Vec<u8>, index: u8) {
    vec.push(BASE_64[index as usize]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_hex_to_base64() {
        fn case(hex: &str, base64: &str) {
            let input: Vec<u8> = hex_to_bytes(hex);
            let output = to_base64(&input);
            let output_string = String::from_utf8(output).expect("Not UTF-8");
            assert_eq!(output_string, base64);
        }

        case(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
        case(
            "aa",
            "qg"
        );
        case(
            "aabb",
            "qrs"
        );
        case(
            "aabbcc",
            "qrvM"
        );
    }


    fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
        let input_chars: Vec<_> = hex_string.chars().collect();

        input_chars.chunks(2).map(|chunk| {
            let first_byte = chunk[0].to_digit(16).unwrap();
            let second_byte = chunk[1].to_digit(16).unwrap();
            ((first_byte << 4) | second_byte) as u8
        }).collect()
    }
}
