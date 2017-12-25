extern crate data_encoding;

use std::io::{self, Read, Write};
use data_encoding::{DecodeError, BASE64, HEXLOWER};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).expect("Error reading input!");
    let output = hex_to_b64(&input).expect("Invalid input!");
    io::stdout().write_all(output.as_bytes()).expect("Error writing to output!");
}

pub fn hex_to_b64(input: &[u8]) -> Result<String, DecodeError> {
    Ok(BASE64.encode(&HEXLOWER.decode(input)?))
}

pub fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.iter().zip(right.iter()).map(|(&a, &b)| a ^ b).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_b64() {
        let input = b"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(hex_to_b64(input).unwrap(), expected);
    }

    #[test]
    fn test_xor() {
        let left = HEXLOWER.decode(b"1c0111001f010100061a024b53535009181c").unwrap();
        let right = HEXLOWER.decode(b"686974207468652062756c6c277320657965").unwrap();
        let expected = HEXLOWER.decode(b"746865206b696420646f6e277420706c6179").unwrap();
        assert_eq!(xor(&left, &right), expected);
    }
}