#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate lazy_static;
extern crate data_encoding;

use std::iter;
use std::str;
use data_encoding::{DecodeError, BASE64, HEXLOWER};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


/*
E 	21912 	  	E 	12.02
T 	16587 	  	T 	9.10
A 	14810 	  	A 	8.12
O 	14003 	  	O 	7.68
I 	13318 	  	I 	7.31
N 	12666 	  	N 	6.95
S 	11450 	  	S 	6.28
R 	10977 	  	R 	6.02
H 	10795 	  	H 	5.92
D 	7874 	  	D 	4.32
L 	7253 	  	L 	3.98
U 	5246 	  	U 	2.88
C 	4943 	  	C 	2.71
M 	4761 	  	M 	2.61
F 	4200 	  	F 	2.30
Y 	3853 	  	Y 	2.11
W 	3819 	  	W 	2.09
G 	3693 	  	G 	2.03
P 	3316 	  	P 	1.82
B 	2715 	  	B 	1.49
V 	2019 	  	V 	1.11
K 	1257 	  	K 	0.69
X 	315 	  	X 	0.17
Q 	205 	  	Q 	0.11
J 	188 	  	J 	0.10
Z 	128 	  	Z 	0.07
*/

#[derive(Debug, Clone)]
struct DecryptionCandidate<'c, 'k> {
    pub ciphertext: &'c [u8],
    pub key: &'k [u8],
    pub plaintext: Vec<u8>,
    pub score: u32,
}

fn main() {
    let reader = BufReader::new(File::open("4.txt").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let ciphertexts: Vec<_> = lines.map(|l| HEXLOWER.decode(l.as_bytes()).expect("Couldn't parse line.")).collect();
    let keys: Vec<_> = (0..=255).map(|k| [k]).collect();
    let mut candidates = Vec::new();
    for ct in &ciphertexts {
        for key in &keys {
            candidates.push(try_decrypt(ct, key));
        }
    }
    let best_match = candidates.iter().max_by_key(|c| c.score).unwrap();
    println!("{:?} {:?}", best_match, str::from_utf8(&best_match.plaintext));
}

pub fn hex_to_b64(input: &[u8]) -> Result<String, DecodeError> {
    Ok(BASE64.encode(&HEXLOWER.decode(input)?))
}

pub fn extend_key<'k>(key: &'k [u8], length: usize) -> impl Iterator<Item=&'k u8> {
    key.iter().cycle().take(length)
}

pub fn xor<'l, 'r, T, U>(left: T, right: U) -> impl Iterator<Item=u8> where T: Iterator<Item=&'l u8>, U: Iterator<Item=&'r u8> {
    left.zip(right).map(|(&a, &b)| a ^ b)
}

fn try_decrypt<'c, 'k>(ciphertext: &'c [u8], key: &'k [u8]) -> DecryptionCandidate<'c, 'k> {
    let extended = extend_key(key, ciphertext.len());
    let plaintext: Vec<_> = xor(ciphertext.iter(), extended).collect();
    let score = score_english(&plaintext);
    DecryptionCandidate { ciphertext, key, plaintext, score }
}

pub fn score_english(input: &[u8]) -> u32 {
    match str::from_utf8(input) {
        Err(_) => 0,
        Ok(text) => {
            text.chars().filter(|ch| ch.is_alphabetic() || ch.is_whitespace()).count() as u32
        }
    }
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
        assert_eq!(xor(left.iter(), right.iter()).collect::<Vec<u8>>(), expected);
    }

    #[test]
    fn test_repeating_xor() {
        let ciphertext = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".as_bytes();
        let key = extend_key("ICE".as_bytes(), ciphertext.len());
        let expected = HEXLOWER.decode(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        assert_eq!(xor(ciphertext.iter(), key).collect::<Vec<u8>>(), expected);
    }
}