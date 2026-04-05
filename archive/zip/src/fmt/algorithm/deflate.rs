use std::fs::File;
use std::io::Read;
use crate::reader::BitReader;

const LENGTH_CODE_ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

pub fn analyze_file<R: Read>(r: &mut BitReader<R>, out: &mut File) {
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
                .map(|(i, len)| (LENGTH_CODE_ORDER[i], len))
                .filter(|&(_, len)| len > 0)
                .collect::<Vec<_>>();

            println!("num_literal_length_codes: {}, num_distance_codes: {}, num_code_length_codes: {}",
                num_literal_length_codes, num_distance_codes, num_code_length_codes);
            println!("code_length_codes: {:?}", code_length_codes);
        }

        _ => panic!("Invalid block type: {:#b}", block_type),
    }
}

#[derive(Clone, Eq, PartialEq)]
struct CodeLength {
    symbol: u16,
    length: u8,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct CodeTableRecord {
    symbol: u16,
    length: u8,
    code: u32,
}

fn build_code_table(code_lengths: &[CodeLength]) -> Vec<CodeTableRecord> {
    let max_length = code_lengths
        .iter()
        .map(|cl| cl.length)
        .max()
        .unwrap_or(0);

    let mut code_table = Vec::with_capacity(1 << max_length);

    let mut code_lengths = code_lengths.to_vec();
    code_lengths.sort_by_key(|cl| (cl.length, cl.symbol));

    let mut code = 0;
    let mut prev_length = code_lengths[0].length;
    for cl in code_lengths {
        if cl.length > prev_length {
            code <<= (cl.length - prev_length) as u32;
            prev_length = cl.length;
        }

        code_table.push(CodeTableRecord {
            symbol: cl.symbol,
            length: cl.length,
            code,
        });

        code += 1;
    }

    code_table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_code_table() {
        let code_lengths = vec![
            CodeLength { symbol: 0, length: 3 },
            CodeLength { symbol: 1, length: 3 },
            CodeLength { symbol: 2, length: 3 },
            CodeLength { symbol: 3, length: 3 },
            CodeLength { symbol: 4, length: 3 },
            CodeLength { symbol: 5, length: 2 },
            CodeLength { symbol: 6, length: 4 },
        ];

        let code_table = build_code_table(&code_lengths);
        assert_eq!(code_table.len(), code_lengths.len());
        assert_eq!(code_table[0], CodeTableRecord{
            symbol: 5,
            length: 2,
            code: 0b00,
        });
        assert_eq!(code_table[1], CodeTableRecord{
            symbol: 0,
            length: 3,
            code: 0b010,
        });
        assert_eq!(code_table[2], CodeTableRecord{
            symbol: 1,
            length: 3,
            code: 0b011,
        });
        assert_eq!(code_table[3], CodeTableRecord{
            symbol: 2,
            length: 3,
            code: 0b100,
        });
        assert_eq!(code_table[4], CodeTableRecord{
            symbol: 3,
            length: 3,
            code: 0b101,
        });
        assert_eq!(code_table[5], CodeTableRecord{
            symbol: 4,
            length: 3,
            code: 0b110,
        });
        assert_eq!(code_table[6], CodeTableRecord{
            symbol: 6,
            length: 4,
            code: 0b1110,
        });
    }
}