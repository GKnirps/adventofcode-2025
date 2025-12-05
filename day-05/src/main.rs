#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;

    let (ranges, ids) = parse(&content)?;

    let fresh_ingredients = count_fresh_ingredients(&ids, &ranges);
    println!("{fresh_ingredients} are fresh");

    Ok(())
}

fn count_fresh_ingredients(ids: &[u64], ranges: &[(u64, u64)]) -> usize {
    ids.iter()
        .filter(|id| ingredient_valid(**id, ranges))
        .count()
}

fn ingredient_valid(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|(from, to)| id >= *from && id <= *to)
}

fn parse(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>), String> {
    let (ranges, ids) = input
        .split_once("\n\n")
        .ok_or_else(|| "unable to split ranges from IDs".to_string())?;
    let ranges: Vec<(u64, u64)> = ranges.lines().map(parse_range).collect::<Result<_, _>>()?;
    let ids: Vec<u64> = ids
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .map_err(|e| format!("unable to parse ID '{line}: {e}"))
        })
        .collect::<Result<_, _>>()?;

    Ok((ranges, ids))
}

fn parse_range(line: &str) -> Result<(u64, u64), String> {
    let (from, to) = line
        .split_once('-')
        .ok_or_else(|| format!("unable to split range line '{line}'"))?;
    let from: u64 = from
        .parse()
        .map_err(|e| format!("unable to parse '{from}': {e}"))?;
    let to: u64 = to
        .parse()
        .map_err(|e| format!("unable to parse '{to}': {e}"))?;
    Ok((from, to))
}

#[cfg(test)]
mod test {
    use super::*;
}
