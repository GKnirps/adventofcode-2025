#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let ids = parse(&content)?;

    let invalid_sum = sum_invalid(&ids);
    println!("The sum of invalid IDs is {invalid_sum}.");

    Ok(())
}

fn sum_invalid(ids: &[(u64, u64)]) -> u64 {
    ids.iter()
        .flat_map(|(from, to)| (*from..=*to).filter(|id| is_invalid(*id)))
        .sum()
}

fn is_invalid(id: u64) -> bool {
    if id == 0 {
        return false;
    }
    let p = 10u64.pow(id.ilog10().div_ceil(2));
    id % p == id / p
}

fn parse(input: &str) -> Result<Vec<(u64, u64)>, String> {
    input.trim().split(',').map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<(u64, u64), String> {
    let (from, to) = line
        .split_once('-')
        .ok_or_else(|| format!("line '{line}' does not contain a dash"))?;
    Ok((
        from.parse::<u64>()
            .map_err(|e| format!("unable to parse ID '{from}': {e}"))?,
        to.parse::<u64>()
            .map_err(|e| format!("unable to parse ID '{to}': {e}"))?,
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn sum_invalid_works_for_example() {
        // given
        let ids = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let sum = sum_invalid(&ids);

        // then
        assert_eq!(sum, 1227775554);
    }
}
