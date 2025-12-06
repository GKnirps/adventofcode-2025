#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (number_lines, op_line) = parse(&content)?;

    let total = grand_total(&number_lines, &op_line);
    println!("The grand total of all operations is {total}");

    Ok(())
}

fn grand_total(numbers: &[Vec<u64>], ops: &[Operator]) -> u64 {
    ops.iter()
        .enumerate()
        .map(|(i, op)| match op {
            Operator::Add => numbers.iter().map(|line| line[i]).sum::<u64>(),
            Operator::Mul => numbers.iter().map(|line| line[i]).product::<u64>(),
        })
        .sum()
}

fn parse(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>), String> {
    let number_lines: Vec<Vec<u64>> = input
        .lines()
        .filter(|line| !line.starts_with('+') && !line.starts_with('*'))
        .map(parse_number_line)
        .collect::<Result<_, _>>()?;
    let op_line: Vec<Operator> = input
        .lines()
        .find(|line| line.starts_with('+') || line.starts_with('*'))
        .map(parse_op_line)
        .unwrap_or_else(|| Err("missing operator line".to_string()))?;

    if number_lines.is_empty() {
        Err("missing number lines".to_string())
    } else if number_lines.iter().any(|line| line.len() != op_line.len()) {
        Err("nor all lines have the same length".to_string())
    } else {
        Ok((number_lines, op_line))
    }
}

fn parse_number_line(line: &str) -> Result<Vec<u64>, String> {
    line.split_whitespace()
        .map(|s| {
            s.parse::<u64>()
                .map_err(|e| format!("unable to parse '{s}': {e}"))
        })
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operator {
    Add,
    Mul,
}

fn parse_op_line(line: &str) -> Result<Vec<Operator>, String> {
    line.split_whitespace()
        .map(|s| match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            _ => Err(format!("unknown operator: '{s}'")),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
}
