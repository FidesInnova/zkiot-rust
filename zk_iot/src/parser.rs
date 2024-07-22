use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};
use anyhow::{Result, anyhow};

use crate::{Gate, GateType};


fn parse_line(line: &str, index: usize) -> Result<(&str, Vec<&str>)> {
    let parts: Vec<&str> = line
        .trim()
        .split(&[',', ' '])
        .filter(|s| !s.trim().is_empty())
        .collect();
    if parts.len() >= 6 {
        Ok((parts[2], parts[3..].to_vec()))
    } else {
        Err(anyhow!("a problem occurred in line {}", index))
    }
}

pub fn parse_from_lines(line_file: &PathBuf, opcodes_file: &PathBuf) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    let line_file = open_file(line_file).unwrap();

    for line in line_file.lines() {
        let line_num = line.unwrap().trim().parse::<usize>().unwrap();
        let gates_file = open_file(opcodes_file).unwrap();
        let line = gates_file.lines().nth(line_num - 1).unwrap().unwrap();
        let (operation, operands) = parse_line(&line, line_num).unwrap();
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // return Err(e);
            eprintln!("Error: {}", e);
            continue;
        }
        let constant = operands.get(2).unwrap().parse::<u64>().unwrap();
        let gate = Gate::new(line_num, 0, None, Some(constant), gate_type.unwrap());
        gates.push(gate);
    }

    Ok(gates)
}

pub fn parser(file_path: PathBuf) -> Result<Vec<Gate>> {
    let reader = open_file(&file_path)?;
    let gates = read_parse_lines(reader)?;
    Ok(gates)
}

fn open_file(file_path: &PathBuf) -> Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}

fn read_parse_lines(reader: BufReader<File>) -> Result<Vec<Gate>> {
    let mut gates = Vec::new();

    for (index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        let (operation, operands) = parse_line(&line, index)?;
        // let gate_type = gate_type(operation)?;
        let gate_type = gate_type(operation);
        if let Err(ref e) = gate_type {
            // return Err(e);
            eprintln!("{}", e);
            continue;
        }

        let constant = operands.get(2).unwrap().parse::<u64>()?;
        let gate = Gate::new(index + 1, 0, None, Some(constant), gate_type?);
        gates.push(gate);
    }

    Ok(gates)
}

fn gate_type(op: &str) -> Result<GateType> {
    match op {
        "mul" => Ok(GateType::Mul),
        "addi" => Ok(GateType::Add),
        _ => Err(anyhow!("operation is not support: {}", op)),
    }
}




#[cfg(test)]
mod parser_test {
    use super::*;

    #[test]
    fn parse_line_func() {
        let line1 = "40380552:       02f407b3                mul     a1,s0,5";
        let line2 = "40380552:       02f407b3                mul     a1, s0, 5";
        let line3 = "40380552:       02f407b3                mul     a1  ,  s0  ,  5  ";

        let parse1 = parse_line(line1, 1).unwrap();
        let parse2 = parse_line(line2, 2).unwrap();
        let parse3 = parse_line(line3, 3).unwrap();

        assert_eq!(parse1, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse2, ("mul", ["a1", "s0", "5"].to_vec()));
        assert_eq!(parse3, ("mul", ["a1", "s0", "5"].to_vec()));
    }
}
