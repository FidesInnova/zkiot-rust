// Copyright 2024 Fidesinnova, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::iter::FromIterator;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn write_vector_to_file(vector: &[u128], filename: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let content = vector.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ");
    file.write_all(content.as_bytes())
}

fn parse_file(path: &str) -> io::Result<Vec<(String, Vec<String>)>> {
    let mut parsed_lines = Vec::new();
    let file = File::open(path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?.trim().to_string();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let instruction = parts[0].to_string();
        let operands = parts[1..]
            .iter()
            .map(|&op| op.trim_end_matches(',').to_string())
            .collect();
        parsed_lines.push((instruction, operands));
    }
    Ok(parsed_lines)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let p = args[1].trim();
    let p: u128 = p.parse().unwrap();


    let mut register_map: HashMap<String, u128> = HashMap::from_iter(vec![
        ("zero".to_owned(), 0), 
        ("ra".to_owned(), 1), 
        ("sp".to_owned(), 2), 
        ("gp".to_owned(), 3), 
        ("tp".to_owned(), 4),
        ("t0".to_owned(), 5), 
        ("t1".to_owned(), 6), 
        ("t2".to_owned(), 7), 
        ("s0".to_owned(), 8), 
        ("s1".to_owned(), 9),
        ("a0".to_owned(), 10), 
        ("a1".to_owned(), 11), 
        ("a2".to_owned(), 12), 
        ("a3".to_owned(), 13), 
        ("a4".to_owned(), 14),
        ("a5".to_owned(), 15), 
        ("a6".to_owned(), 16), 
        ("a7".to_owned(), 17), 
        ("s2".to_owned(), 18), 
        ("s3".to_owned(), 19),
        ("s4".to_owned(), 20), 
        ("s5".to_owned(), 21), 
        ("s6".to_owned(), 22), 
        ("s7".to_owned(), 23), 
        ("s8".to_owned(), 24),
        ("s9".to_owned(), 25), 
        ("s10".to_owned(), 26), 
        ("s11".to_owned(), 27), 
        ("t3".to_owned(), 28), 
        ("t4".to_owned(), 29),
        ("t5".to_owned(), 30), 
        ("t6".to_owned(), 31),
    ]);

    let parsed_data = parse_file("program.s")?;

    let mut w = Vec::new();
    for (inst, reg) in parsed_data {
        match inst.as_str() {
            "addi" => {
                let (dest, src, imm) = (reg[0].clone(), reg[1].clone(), reg[2].clone());
                if let (Some(&src_val), Ok(imm_val)) = (register_map.get(src.as_str()), imm.parse::<u64>()) {
                    register_map.insert(dest, (src_val + imm_val as u128) % p);
                }
            }
            "add" => {
                let (dest, src1, src2) = (reg[0].clone(), reg[1].clone(), reg[2].clone());
                if let (Some(&val1), Some(&val2)) = (register_map.get(src1.as_str()), register_map.get(src2.as_str())) {
                    register_map.insert(dest, (val1 + val2) % p);
                }
            }
            "mul" => {
                let (dest, src1, src2) = (reg[0].clone(), reg[1].clone(), reg[2].clone());
                if let (Some(&val1), Some(&val2)) = (register_map.get(src1.as_str()), register_map.get(src2.as_str())) {
                    register_map.insert(dest, (val1 * val2) % p);
                }
            }
            _ => {}
        }

        if let Some(&result) = register_map.get(&reg[0].clone()) {
            w.push((reg[0].clone(), result % p));
        }
    }


    let w: Vec<u128> = w.iter().map(|(_, v)| *v).collect();


    let mut z = vec![1];
    let x: Vec<u128> = (0..=31).collect();
    z.extend(&x);
    z.extend(&w);


    println!("Register Map: {:?}", register_map);
    println!("X: {:?}", x);
    println!("W: {:?}", w);
    println!("Z: {:?}", z);
    write_vector_to_file(&z, "proof_generation/z_vec.txt")
}