#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (start, splitters) = parse(&content)?;

    let n_splits = count_splits(start, &splitters);
    println!("The tachyon beam is split a total of {n_splits} times.");

    Ok(())
}

fn count_splits(start: Pos, splitters: &HashSet<Pos>) -> u32 {
    let max_y: isize = splitters
        .iter()
        .map(|(_, y)| y)
        .max()
        .copied()
        .unwrap_or(start.1);
    let mut visited: HashSet<Pos> = HashSet::with_capacity(splitters.len() * 10);
    let mut stack: Vec<Pos> = Vec::with_capacity(splitters.len() * 2);
    let mut count: u32 = 0;

    stack.push(start);
    while let Some((x, y)) = stack.pop() {
        if y > max_y || visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if splitters.contains(&(x, y + 1)) {
            stack.push((x - 1, y + 1));
            stack.push((x + 1, y + 1));
            count += 1;
        } else {
            stack.push((x, y + 1));
        }
    }
    count
}

type Pos = (isize, isize);

fn parse(input: &str) -> Result<(Pos, HashSet<Pos>), String> {
    let start: Pos = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            line.chars()
                .enumerate()
                .find(|(_, c)| *c == 'S')
                .map(|(x, _)| (x as isize, y as isize))
        })
        .next()
        .ok_or_else(|| "unable to find start in input".to_string())?;
    let splitters: HashSet<Pos> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '^')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect();

    Ok((start, splitters))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

    #[test]
    fn count_splits_works_for_example() {
        // given
        let (start, splitters) = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let count = count_splits(start, &splitters);

        // then
        assert_eq!(count, 21);
    }
}
