#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let warehouse = parse(&content)?;

    let accessible_rolls = count_accessible_rolls(&warehouse);
    println!("{accessible_rolls} rolls of paper can be accessed by a forklift");

    Ok(())
}

fn count_accessible_rolls(warehouse: &Warehouse) -> usize {
    warehouse
        .tiles
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile)
        .filter(|(i, _)| {
            let x = i % warehouse.width;
            let y = i / warehouse.width;
            warehouse.count_neighbours(x, y) < 4
        })
        .count()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Warehouse {
    tiles: Vec<bool>,
    width: usize,
    height: usize,
}

impl Warehouse {
    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if x < self.width && y < self.height {
            self.tiles.get(x + y * self.width).copied()
        } else {
            None
        }
    }
    fn count_neighbours(&self, x: usize, y: usize) -> usize {
        let mut count: usize = 0;
        if x > 0 && y > 0 {
            count += self.get(x - 1, y - 1).unwrap_or(false) as usize;
        }
        if x > 0 {
            count += self.get(x - 1, y).unwrap_or(false) as usize;
            count += self.get(x - 1, y + 1).unwrap_or(false) as usize;
        }
        if y > 0 {
            count += self.get(x, y - 1).unwrap_or(false) as usize;
            count += self.get(x + 1, y - 1).unwrap_or(false) as usize;
        }
        count += self.get(x + 1, y).unwrap_or(false) as usize;
        count += self.get(x, y + 1).unwrap_or(false) as usize;
        count += self.get(x + 1, y + 1).unwrap_or(false) as usize;
        count
    }
}

fn parse(input: &str) -> Result<Warehouse, String> {
    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .ok_or_else(|| "unexpected empty input".to_string())?
        .len();
    if !input.lines().all(|line| line.len() == width) {
        return Err("line lengths are not uniform".to_string());
    }
    let tiles: Vec<bool> = input
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '.' => Ok(false),
            '@' => Ok(true),
            _ => Err(format!("unknown tile: '{c}'")),
        })
        .collect::<Result<_, _>>()?;
    Ok(Warehouse {
        tiles,
        width,
        height,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn count_accessible_rolls_works_for_example() {
        // given
        let warehouse = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let count = count_accessible_rolls(&warehouse);

        // then
        assert_eq!(count, 13);
    }
}
