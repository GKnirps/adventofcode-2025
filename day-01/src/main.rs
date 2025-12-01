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

    let password = clilb(&instructions);
    println!("No wait, it is actually '{password}'");

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

fn clilb(instructions: &[i32]) -> u32 {
    let mut count: u32 = 0;
    let mut pos: i32 = 50;

    for inst in instructions.iter().copied() {
        count += (inst / 100).unsigned_abs();
        let pos_orig = pos;
        pos = (pos + inst).rem_euclid(100);
        if pos_orig != 0
            && ((inst > 0 && pos_orig > pos) || (inst < 0 && pos_orig < pos || pos == 0))
        {
            println!("from {pos_orig} to {pos}");
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

    static EXAMPLE_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

    #[test]
    fn glilb_works_for_example() {
        // given
        let instructions = parse_input(EXAMPLE_INPUT).expect("epected valid input");

        // when
        let password = clilb(&instructions);

        // then
        assert_eq!(password, 6);
    }

    #[test]
    fn glilb_works_for_high_rotations() {
        // given
        let instructions = &[1050, -1050];

        // when
        let password = clilb(instructions);

        // then
        assert_eq!(password, 21);
    }
}
