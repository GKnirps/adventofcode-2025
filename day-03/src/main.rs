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

    let joltage = max_joltage_sum(&banks, 2);
    println!("The total output of joltage is {joltage}.");

    let joltage = max_joltage_sum(&banks, 12);
    println!("The total output of high joltage is {joltage}.");

    Ok(())
}

fn max_joltage_sum(banks: &[Vec<u64>], n: u8) -> u64 {
    banks
        .iter()
        .map(|bank| max_joltage_for_bank(bank, n, 0))
        .sum()
}

fn max_joltage_for_bank(bank: &[u64], n: u8, current: u64) -> u64 {
    if bank.len() < n.into() {
        0
    } else if n > 1 {
        let mut max_digit = 0;
        let mut digit_pos = 0;

        for (i, digit) in bank[..(bank.len() - ((n as usize) - 1))].iter().enumerate() {
            if *digit > max_digit {
                max_digit = *digit;
                digit_pos = i;
            }
        }
        max_joltage_for_bank(&bank[digit_pos + 1..], n - 1, current * 10 + max_digit)
    } else {
        let max_digit = bank.iter().max().copied().unwrap_or(0);
        current * 10 + max_digit
    }
}

fn parse(input: &str) -> Result<Vec<Vec<u64>>, String> {
    input.lines().map(parse_bank).collect()
}

fn parse_bank(line: &str) -> Result<Vec<u64>, String> {
    line.chars()
        .map(|c| {
            c.to_digit(10)
                .map(|digit| digit as u64)
                .ok_or_else(|| format!("invalid joltage: '{c}'"))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
}
