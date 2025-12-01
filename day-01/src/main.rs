#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let instructions = parse_input(&content)?;

    let password = rot_and_count_0(&instructions);
    println!("The password is '{password}'");

    Ok(())
}

fn rot_and_count_0(instructions: &[i32]) -> u32 {
    let mut count: u32 = 0;
    let mut pos: i32 = 50;

    for inst in instructions {
        pos = (pos + inst).rem_euclid(100);
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<i32, String> {
    if let Some(n) = line.strip_prefix('R') {
        n.parse()
            .map_err(|e| format!("unable to parse line '{line}': {e})"))
    } else if let Some(n) = line.strip_prefix('L') {
        n.parse::<i32>()
            .map(|l| -l)
            .map_err(|e| format!("unable to parse line '{line}': {e})"))
    } else {
        Err(format!("line '{line}' does not start wth 'R' or 'L'"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
