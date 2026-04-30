use crate::reader::BitReader;
use crate::writer::Writer;
use std::io::Read;

const LENGTH_CODE_ORDER: [u16; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

// TODO: multiple blocks
pub fn analyze_file<R: Read>(r: &mut BitReader<R>, w: &mut Writer) {
    let is_final = r.read_bits(1);
    let block_type = r.read_bits(2);

    match block_type {
        0b00 => {
            // stored
        }

        0b01 => {
            // fixed huffman
        }

        0b10 => {
            let num_literal_length_codes = r.read_bits(5) + 257;
            let num_distance_codes = r.read_bits(5) + 1;
            let num_code_length_codes = r.read_bits(4) + 4;

            let code_length_codes = (0..num_code_length_codes)
                .map(|_| r.read_bits(3) as u8)
                .enumerate()
                .map(|(i, len)| CodeLength {
                    symbol: LENGTH_CODE_ORDER[i],
                    length: len,
                })
                .filter(|cl| cl.length > 0)
                .collect::<Vec<_>>();

            // println!("num_literal_length_codes: {}, num_distance_codes: {}, num_code_length_codes: {}",
            //     num_literal_length_codes, num_distance_codes, num_code_length_codes);
            // println!("code_length_codes: {:?}", code_length_codes);

            let code_length_code_table = build_code_table(&code_length_codes);
            for i in 0..code_length_code_table.len() {
                println!("code: {:03b}, symbol: {}, length: {}",
                    i, code_length_code_table[i].symbol, code_length_code_table[i].length);
            }

            let max_length = code_length_codes.iter().map(|cl| cl.length).max().unwrap_or(0);
            let mut literal_length_codes = Vec::new();
            let mut i = 0;
            while i < num_literal_length_codes {
                let code = r.peek_bits_rev(max_length as usize);
                let code_length_code = &code_length_code_table[code as usize];

                _ = r.read_bits(code_length_code.length as usize);

                match code_length_code.symbol {
                    0 => {
                        i += 1;
                    }

                    1..=15 => {
                        literal_length_codes.push(CodeLength {
                            symbol: i as u16,
                            length: code_length_code.symbol as u8,
                        });
                        i += 1;
                    }

                    // copy previous code length 3-6 times
                    16 => {
                        let copy_length = r.read_bits(2) + 3;
                        for _ in 0..copy_length {
                            literal_length_codes.push(CodeLength {
                                symbol: i as u16,
                                length: code_length_code.symbol as u8,
                            });
                            i += 1;
                        }
                    }

                    // copy 0 for 3-10 times
                    17 => {
                        let copy_length = r.read_bits(3) + 3;
                        i += copy_length;
                    }

                    // copy 0 for 11-138 times
                    18 => {
                        let copy_length = r.read_bits(7) + 11;
                        i += copy_length;
                    }

                    _ => panic!("Invalid code length code symbol: {}", code_length_code.symbol),
                }
            }

            println!("literal_length_codes: {:?}", literal_length_codes);

            i = 0;
            let mut distance_codes = Vec::new();
            while i < num_distance_codes {
                let code = r.peek_bits_rev(max_length as usize);
                let code_length_code = &code_length_code_table[code as usize];

                _ = r.read_bits(code_length_code.length as usize);

                match code_length_code.symbol {
                    0 => {
                        i += 1;
                    }

                    1..=15 => {
                        distance_codes.push(CodeLength {
                            symbol: i as u16,
                            length: code_length_code.symbol as u8,
                        });
                        i += 1;
                    }

                    // copy previous code length 3-6 times
                    16 => {
                        let copy_length = r.read_bits(2) + 3;
                        for _ in 0..copy_length {
                            distance_codes.push(CodeLength {
                                symbol: i as u16,
                                length: code_length_code.symbol as u8,
                            });
                            i += 1;
                        }
                    }

                    // copy 0 for 3-10 times
                    17 => {
                        let copy_length = r.read_bits(3) + 3;
                        i += copy_length;
                    }

                    // copy 0 for 11-138 times
                    18 => {
                        let copy_length = r.read_bits(7) + 11;
                        i += copy_length;
                    }

                    _ => panic!("Invalid code length code symbol: {}", code_length_code.symbol),
                }
            }

            println!("distance_codes: {:?}", distance_codes);

            // TODO: distanceが15になったときはまずい
            let literal_code_table = build_code_table(&literal_length_codes);
            let distance_code_table = build_code_table(&distance_codes);
            let literal_code_table_max_length = literal_length_codes.iter().map(|cl| cl.length).max().unwrap_or(0);
            let distance_code_table_max_length = distance_codes.iter().map(|cl| cl.length).max().unwrap_or(0);

            for i in 0..literal_code_table.len() {
                println!("literal code: {:b}, symbol: {}, length: {}",
                    i, literal_code_table[i].symbol, literal_code_table[i].length);
            }
            for i in 0..distance_code_table.len() {
                println!("distance code: {:b}, symbol: {}, length: {}",
                    i, distance_code_table[i].symbol, distance_code_table[i].length);
            }

            loop {
                let code = r.peek_bits_rev(literal_code_table_max_length as usize);
                let literal_code = &literal_code_table[code as usize];

                _ = r.read_bits(literal_code.length as usize);

                match literal_code.symbol {
                    0..=255 => {
                        println!("literal: {} {}", literal_code.symbol as u8 as char, literal_code.symbol);
                        w.write_u8(literal_code.symbol as u8);
                    }

                    256 => {
                        // end of block
                        break;
                    }

                    257..=285 => {
                        let length_code = &LENGTH_CODES[literal_code.symbol as usize - 257];
                        let additional = r.read_bits(length_code.bits as usize) as u16;
                        let length = length_code.offset + additional;

                        let distance_code = {
                            let code = r.peek_bits_rev(distance_code_table_max_length as usize);
                            let distance_code = &distance_code_table[code as usize];
                            _ = r.read_bits(distance_code.length as usize);
                            distance_code
                        };

                        let distance_length_code = &DISTANCE_CODES[distance_code.symbol as usize];
                        let additional = r.read_bits(distance_length_code.bits as usize) as u16;
                        let distance = distance_length_code.offset + additional;

                        println!("copy: length={}, distance={}", length, distance);
                        w.copy(distance as u64, length as u64);
                    }
                    _ => panic!("Invalid literal/length code: {}", code),
                }
            }
        }

        _ => panic!("Invalid block type: {:#b}", block_type),
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct CodeLength {
    symbol: u16,
    length: u8,
}

fn build_code_table(code_lengths: &[CodeLength]) -> Vec<CodeLength> {
    let max_length = code_lengths.iter().map(|cl| cl.length).max().unwrap_or(0);

    let mut code_table = vec![
        CodeLength { symbol: 0, length: 0 };
        1 << max_length
    ];

    let mut code_lengths = code_lengths.to_vec();
    code_lengths.sort_by_key(|cl| (cl.length, cl.symbol));

    let mut code = 0;
    for cl in code_lengths {
        for i in 0..(1 << (max_length - cl.length)) {
            code_table[code] = cl.clone();
            code += 1;
        }
    }

    code_table
}

struct CodeRecord {
    bits: u8,
    offset: u16,
}

// start with 257
const LENGTH_CODES: [CodeRecord; 29] = [
    CodeRecord { bits: 0, offset: 3 },
    CodeRecord { bits: 0, offset: 4 },
    CodeRecord { bits: 0, offset: 5 },
    CodeRecord { bits: 0, offset: 6 },
    CodeRecord { bits: 0, offset: 7 },
    CodeRecord { bits: 0, offset: 8 },
    CodeRecord { bits: 0, offset: 9 },
    CodeRecord { bits: 0, offset: 10 },
    CodeRecord { bits: 1, offset: 11 },
    CodeRecord { bits: 1, offset: 13 },
    CodeRecord { bits: 1, offset: 15 },
    CodeRecord { bits: 1, offset: 17 },
    CodeRecord { bits: 2, offset: 19 },
    CodeRecord { bits: 2, offset: 23 },
    CodeRecord { bits: 2, offset: 27 },
    CodeRecord { bits: 2, offset: 31 },
    CodeRecord { bits: 3, offset: 35 },
    CodeRecord { bits: 3, offset: 43 },
    CodeRecord { bits: 3, offset: 51 },
    CodeRecord { bits: 3, offset: 59 },
    CodeRecord { bits: 4, offset: 67 },
    CodeRecord { bits: 4, offset: 83 },
    CodeRecord { bits: 4, offset: 99 },
    CodeRecord { bits: 4, offset: 115 },
    CodeRecord { bits: 5, offset: 131 },
    CodeRecord { bits: 5, offset: 163 },
    CodeRecord { bits: 5, offset: 195 },
    CodeRecord { bits: 5, offset: 227 },
    CodeRecord { bits: 0, offset: 258 },
];

// start with 0
const DISTANCE_CODES: [CodeRecord; 30] = [
    CodeRecord { bits: 0, offset: 1 },
    CodeRecord { bits: 0, offset: 2 },
    CodeRecord { bits: 0, offset: 3 },
    CodeRecord { bits: 0, offset: 4 },
    CodeRecord { bits: 1, offset: 5 },
    CodeRecord { bits: 1, offset: 7 },
    CodeRecord { bits: 2, offset: 9 },
    CodeRecord { bits: 2, offset: 13 },
    CodeRecord { bits: 3, offset: 17 },
    CodeRecord { bits: 3, offset: 25 },
    CodeRecord { bits: 4, offset: 33 },
    CodeRecord { bits: 4, offset: 49 },
    CodeRecord { bits: 5, offset: 65 },
    CodeRecord { bits: 5, offset: 97 },
    CodeRecord { bits: 6, offset: 129 },
    CodeRecord { bits: 6, offset: 193 },
    CodeRecord { bits: 7, offset: 257 },
    CodeRecord { bits: 7, offset: 385 },
    CodeRecord { bits: 8, offset: 513 },
    CodeRecord { bits: 8, offset: 769 },
    CodeRecord { bits: 9, offset: 1025 },
    CodeRecord { bits:10 , offset :1537},
    CodeRecord {bits :10 ,offset :2049},
    CodeRecord {bits :11 ,offset :3073},
    CodeRecord{bits :11 ,offset :4097},
    CodeRecord{bits :12 ,offset :6145},
    CodeRecord{bits :12 ,offset :8193},
    CodeRecord{bits :13 ,offset :12289},
    CodeRecord{bits :13 ,offset :16385},
    CodeRecord{bits :14 ,offset :24577},
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_code_table() {
        let code_lengths = vec![
            CodeLength { symbol: 0, length: 3 },
            CodeLength { symbol: 1, length: 2 },
            CodeLength { symbol: 2, length: 2 },
            CodeLength { symbol: 3, length: 3 },
            CodeLength { symbol: 4, length: 3 },
        ];

        let code_table = build_code_table(&code_lengths);
        assert_eq!(code_table[0b000], CodeLength { symbol: 1, length: 2 });
        assert_eq!(code_table[0b001], CodeLength { symbol: 1, length: 2 });
        assert_eq!(code_table[0b010], CodeLength { symbol: 2, length: 2 });
        assert_eq!(code_table[0b011], CodeLength { symbol: 2, length: 2 });
        assert_eq!(code_table[0b100], CodeLength { symbol: 0, length: 3 });
        assert_eq!(code_table[0b101], CodeLength { symbol: 3, length: 3 });
        assert_eq!(code_table[0b110], CodeLength { symbol: 4, length: 3 });
        assert_eq!(code_table[0b111], CodeLength { symbol: 0, length: 0 });
    }
}
