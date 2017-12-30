#![feature(inclusive_range_syntax)]
#![feature(conservative_impl_trait)]

extern crate data_encoding;

use std::str;
use data_encoding::{DecodeError, BASE64, HEXLOWER};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


/*
note to self: space is more common than E

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
pub struct DecryptionCandidate<'c, 'k> {
    pub ciphertext: &'c [u8],
    pub key: &'k [u8],
    pub plaintext: Vec<u8>,
    pub score: u32,
}

#[derive(Debug, Clone)]
pub struct KeysizeCandidate {
    pub keysize: usize,
    pub score: f64,
}

fn main() {
    let reader = BufReader::new(File::open("6.txt").expect("Couldn't read input file."));
    let lines: Vec<_> = reader.lines().map(|x| x.expect("Couldn't read line.").trim().to_owned()).collect();
    let ciphertext = BASE64.decode(lines.concat().as_bytes()).expect("Couldn't deserialize ciphertext.");

    // 1. discover keysize via testing block-to-block edit distances
    let keysizes = 1..=40;
    let mut keysize_candidates = Vec::new();
    for keysize in keysizes {
        let blocks: Vec<_> = ciphertext.chunks(keysize).collect();
        let pairs: Vec<_> = blocks.windows(2).collect();
        let mut total_distance = 0f64;
        for pair in &pairs {
            total_distance += edit_distance(pair[0], pair[1]) as f64;
        }
        let score = total_distance / ((keysize * pairs.len()) as f64);
        keysize_candidates.push(KeysizeCandidate { keysize, score });
    }

    let best_candidate = keysize_candidates.iter().min_by(|x, y| x.score.partial_cmp(&y.score).unwrap());
    let best_keysize = best_candidate.unwrap().keysize;
    println!("Testing keysize: {:?}", best_keysize);

    // 2. for each offset from 0..keysize, figure out the key byte at that offset via
    // frequency analysis on plaintext output for every possible key byte
    let mut reconstructed_key = Vec::with_capacity(best_keysize);
    let n_blocks = ciphertext.len() / best_keysize;
    for i in 0..best_keysize {
        let mut plaintext_bytes = Vec::with_capacity(n_blocks);
        for block in ciphertext.chunks(best_keysize) {
            if let Some(b) = block.get(i) {
                plaintext_bytes.push(*b);
            }
        }
        let key_candidates: Vec<_> = (0..=255).map(|k| [k]).collect();
        let mut decryption_candidates = Vec::with_capacity(256);
        for k in &key_candidates {
            let candidate = try_decrypt(&plaintext_bytes, k);
            decryption_candidates.push(candidate);
        }
        let best_decryption_candidate = decryption_candidates.iter().max_by_key(|x| x.score).unwrap();
        reconstructed_key.push(best_decryption_candidate.key[0]);
    }

    println!("Reconstructed key: {:?}", str::from_utf8(&reconstructed_key));

    let decrypted = try_decrypt(&ciphertext, &reconstructed_key);
    println!("Supposed plaintext: {:?}", str::from_utf8(&decrypted.plaintext));
}

pub fn edit_distance(left: &[u8], right: &[u8]) -> u32 {
    xor(left, right).into_iter().map(u8::count_ones).sum()
}

pub fn hex_to_b64(input: &[u8]) -> Result<String, DecodeError> {
    Ok(BASE64.encode(&HEXLOWER.decode(input)?))
}

pub fn extend_key(key: &[u8], length: usize) -> Vec<u8> {
    key.iter().cycle().take(length).cloned().collect()
}

pub fn xor(left: &[u8], right: &[u8]) -> Vec<u8> {
    left.into_iter().zip(right.into_iter()).map(|(a, b)| a ^ b).collect()
}

pub fn try_decrypt<'c, 'k>(ciphertext: &'c [u8], key: &'k [u8]) -> DecryptionCandidate<'c, 'k> {
    let extended: Vec<_> = extend_key(key, ciphertext.len());
    let plaintext: Vec<_> = xor(ciphertext, &extended);
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
    fn test_edit_distance() {
        let left = b"this is a test";
        let right = b"wokka wokka!!!";
        assert_eq!(edit_distance(left, right), 37)
    }

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

    #[test]
    fn test_repeating_xor() {
        let ciphertext = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let key = extend_key(b"ICE", ciphertext.len());
        let expected = HEXLOWER.decode(b"0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f").unwrap();
        assert_eq!(xor(ciphertext, &key), expected);
    }
}
