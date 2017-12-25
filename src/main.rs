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
    let b64_table = assemble_b64_table();
    loop {
        let l = reader.read(&mut buffer)?;
        match buffer[..l].iter().rposition(|&b| (b as char).is_digit(16)) {
            None => { break; } // we read nothing but terminators
            Some(pos) => {     // pos is the index of the last hex digit
                let actual_l = pos + 1;
                writer.write(&print_as_hex(actual_l, &buffer, &b64_table))?;
                if actual_l < l {
                    break;
                }
            }
        }
    }
    Ok(())
}

fn assemble_b64_table() -> [u8; 64] {
    let mut table = [0; 64];
    for i in 0..26 {
        table[i] = i as u8 + 'A' as u8;
    }
    for i in 0..26 {
        table[i + 26] = i as u8 + 'a' as u8
    }
    for i in 0..10 {
        table[i + 52] = i as u8 + '0' as u8;
    }
    table[62] = '+' as u8;
    table[63] = '/' as u8;
    table
}

fn print_as_hex(l: usize, in_buffer: &[u8], b64_table: &[u8]) -> [u8; 4] {
    let mut x: u32 = 0;
    let mut out_buffer = [0; 4];
    for i in 0..6 {
        x = x << 4;
        if i < l {
            let nibble = (in_buffer[i] as char).to_digit(16).expect("Non-hex digit found!");
            x += nibble;
        }
    }
    for i in 0..4 {
        let shift = (3 - i) * 6;
        let shifted = x >> shift;
        let char_val = shifted & 63;

        if l / 2 >= i {
            out_buffer[i] = b64_table[char_val as usize];
        } else {
            out_buffer[i] = '=' as u8;
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
        hex_to_b64(&mut input, &mut output);
        assert_eq!(String::from_utf8(output).unwrap(), "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}