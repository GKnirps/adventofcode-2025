#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let red_tiles = parse(&content)?;

    if let Some(area) = largest_area(&red_tiles) {
        println!("The largest rectangle has an area of {area}");
    } else {
        println!("Not enough red tiles…");
    }

    Ok(())
}

fn largest_area(red_tiles: &[Pos]) -> Option<i64> {
    red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i, (x1, y1))| {
            red_tiles[i + 1..]
                .iter()
                .map(move |(x2, y2)| ((x1 - x2).abs() + 1) * ((y1 - y2).abs() + 1))
        })
        .max()
}

type Pos = (i64, i64);

fn parse(input: &str) -> Result<Vec<Pos>, String> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Result<Pos, String> {
    let (left, right) = line
        .split_once(',')
        .ok_or_else(|| format!("unable to split line '{line}'"))?;
    let x: i64 = left
        .parse()
        .map_err(|e| format!("unable to parse '{left}': {e}"))?;
    let y: i64 = right
        .parse()
        .map_err(|e| format!("unable to parse '{right}': {e}"))?;
    Ok((x, y))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

    #[test]
    fn largest_area_works_for_example() {
        // given
        let tiles = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let result = largest_area(&tiles);

        // then
        assert_eq!(result, Some(50));
    }
}
