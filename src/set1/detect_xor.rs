use std::collections::HashMap;

pub fn english_freq(input: &str) -> HashMap<char, f32> {
    let mut counter: HashMap<char, u32> = HashMap::new();
    let mut input_length: u32 = 0;
    for c in input.chars().filter(|c| c.is_ascii_alphabetic()) {
        let lowercased = c.to_lowercase().next().unwrap();
        counter.entry(lowercased).and_modify(|i| *i += 1).or_insert(1);
        input_length += 1;
    }
    counter.into_iter().map(|(c, n)| (c, n as f32 / input_length as f32)).collect()
}

pub fn freq_diff(a: &HashMap<char, f32>, b: &HashMap<char, f32>) -> f32 {
    a.iter().fold(0f32, |diff, (c, af)| {
        let freq_diff = b.get(c).map_or(*af, |bf| (af - bf).abs());
        diff + freq_diff
    })
}

#[test]
fn detect_single_byte_xor() {
    let mut standard_english_freq: HashMap<char, f32> = HashMap::new();
    standard_english_freq.insert('t', 0.15978);
    standard_english_freq.insert('a', 0.1168);
    standard_english_freq.insert('o', 0.07631);
    standard_english_freq.insert('i', 0.07294);
    standard_english_freq.insert('s', 0.06686);
    standard_english_freq.insert('w', 0.05497);
    standard_english_freq.insert('c', 0.05238);
    standard_english_freq.insert('b', 0.04434);
    standard_english_freq.insert('p', 0.04319);
    standard_english_freq.insert('h', 0.042);
    standard_english_freq.insert('f', 0.04027);
    standard_english_freq.insert('m', 0.03826);
    standard_english_freq.insert('d', 0.03174);
    standard_english_freq.insert('r', 0.02826);
    standard_english_freq.insert('e', 0.02799);
    standard_english_freq.insert('l', 0.02415);
    standard_english_freq.insert('n', 0.02284);
    standard_english_freq.insert('g', 0.01642);
    standard_english_freq.insert('u', 0.01183);
    standard_english_freq.insert('v', 0.00824);
    standard_english_freq.insert('y', 0.00763);
    standard_english_freq.insert('j', 0.00511);
    standard_english_freq.insert('k', 0.00456);
    standard_english_freq.insert('q', 0.00222);
    standard_english_freq.insert('x', 0.00045);
    standard_english_freq.insert('z', 0.00045);

    use std::cmp::Ordering;
    use convert::*;
    use super::xor::*;
    let input = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let mut diffs: Vec<(char, f32, String)> =
        (b'A'..b'Z').map(|b|{
            let xored = byte_xor(&input, b);
            let s = String::from_utf8(xored).unwrap();
            let freq = english_freq(&s);
            let diff = freq_diff(&standard_english_freq, &freq);
            (b as char, diff, s)
        }).collect();
    diffs.sort_by(|&(_, d1, _), &(_, d2, _)| d1.partial_cmp(&d2).unwrap_or(Ordering::Equal));
    let &(c, _, ref s) = diffs.first().unwrap();
    assert_eq!("Cooking MC's like a pound of bacon", s);
    assert_eq!('X', c);
}
