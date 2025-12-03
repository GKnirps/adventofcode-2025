#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let banks = parse(&content)?;

    let joltage = max_joltage_sum(&banks);
    println!("The total output of joltage is {joltage}.");

    Ok(())
}

fn max_joltage_sum(banks: &[Vec<u32>]) -> u32 {
    banks.iter().map(|bank| max_joltage_for_bank(bank)).sum()
}

fn max_joltage_for_bank(bank: &[u32]) -> u32 {
    if bank.len() < 2 {
        return 0;
    }
    let mut max_digit_1 = 0;
    let mut digit_1_pos = 0;
    for (i, digit) in bank[..bank.len() - 1].iter().enumerate() {
        if *digit > max_digit_1 {
            max_digit_1 = *digit;
            digit_1_pos = i;
        }
    }
    let max_digit_2 = bank[digit_1_pos + 1..].iter().max().copied().unwrap_or(0);
    max_digit_1 * 10 + max_digit_2
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, String> {
    input.lines().map(parse_bank).collect()
}

fn parse_bank(line: &str) -> Result<Vec<u32>, String> {
    line.chars()
        .map(|c| {
            c.to_digit(10)
                .ok_or_else(|| format!("invalid joltage: '{c}'"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
}
