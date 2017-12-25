use std::io::{self, BufReader, BufWriter, Read, Write};
use std::error::Error;

fn main() {
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    if let Err(e) = hex_to_b64(&mut reader, &mut writer) {
        println!("=( {}", e);
    }
}

fn hex_to_b64<T, U>(reader: &mut T, writer: &mut U) -> Result<(), Box<Error>> where T: Read, U: Write {
    let mut buffer = [0; 6];
    let alphabet = b64_alphabet();
    loop {
        let l = reader.read(&mut buffer)?;
        match buffer[..l].iter().rposition(|&b| (b as char).is_digit(16)) {
            None => { break; } // we read nothing but terminators
            Some(pos) => {     // pos is the index of the last hex digit
                writer.write(&sextet_to_b64(&buffer[..pos+1], &alphabet))?;
                if pos+1 < l {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn b64_alphabet() -> &'static [u8] {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".as_bytes()
}

fn sextet_to_b64(sextet: &[u8], alphabet: &[u8]) -> [u8; 4] {
    let mut x: u32 = 0;
    let mut out_buffer = [0; 4];
    for i in 0..6 {
        x = x << 4;
        if i < sextet.len() {
            x += (sextet[i] as char).to_digit(16).expect("Non-hex digit found!");
        }
    }
    for i in 0..4 {
        out_buffer[i] = if sextet.len() / 2 >= i {
            let shift = (3 - i) * 6;
            let shifted = x >> shift;
            let char_val = shifted & 63;
            alphabet[char_val as usize]
        } else {
            b'='
        }
    }
    out_buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let mut input = BufReader::new("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".as_bytes());
        let mut output = Vec::new();
        hex_to_b64(&mut input, &mut output).expect("Failed to write output!");
        assert_eq!(String::from_utf8(output).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}