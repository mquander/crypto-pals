use std::io::{self, BufReader, BufWriter, Read, Write};

const NUM_WINDOWS: usize = 128;

fn main() {
    let mut buffer = [0; 6];
    let mut out_buffer = [0; 4];
    let b64_table = assemble_b64_table();
    let mut reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());
    loop {
        match reader.read(&mut buffer) {
            Ok(l) => {
                match buffer[..l].iter().rposition(|&b| (b as char).is_digit(16)) {
                    None => { break; }
                    Some(pos) => {
                        let actual_l = pos + 1;
                        let out_len = print_as_hex(actual_l, &buffer, &mut out_buffer, &b64_table);
                        writer.write(&out_buffer[..out_len]);
                        if actual_l < l {
                            break;
                        }
                    }
                }
            }
            Err(l) => println!("=( {}", l),
        }
    }
}

fn hex_byte_to_nibble(hex_byte: u8) -> u8 {
    if hex_byte > 96 {
        // lower case letter
        hex_byte - 87
    } else if hex_byte > 64 {
        // upper case letter
        hex_byte - 55
    } else {
        // digit
        hex_byte - 48
    }
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

fn print_as_hex(l: usize, in_buffer: &[u8], out_buffer: &mut [u8], b64_table: &[u8]) -> usize {
    let triplet_count = (l + 5) / 6;
    for i in 0..triplet_count {
        let index = i * 6;
        let mut x: u32 = 0;
        for offset in 0..6 {
            x = x << 4;
            let next = index + offset;
            if next < l {
                let nibble = hex_byte_to_nibble(in_buffer[next]) as u32;
                x += nibble;
            }
        }
        for sextet in 0..4 {
            let shift = (3 - sextet) * 6;
            let shifted = x >> shift;
            let char_val = shifted & 63;
            let out_index = i * 4 + sextet ;

            if (l - index) / 2 >= sextet {
                out_buffer[out_index] = b64_table[char_val as usize];
            } else {
                out_buffer[out_index] = '=' as u8;
            }

        }
    }
    triplet_count * 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let mut out_buffer = [0; 4 * NUM_WINDOWS];
        let b64_table = assemble_b64_table();
        let output_len = print_as_hex(input.len(), input.as_bytes(), &mut out_buffer, &b64_table);
        assert_eq!(std::str::from_utf8(&out_buffer[..output_len]).unwrap(), output);
    }
}