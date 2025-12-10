#![forbid(unsafe_code)]

use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let machines = parse(&content)?;

    if let Some(n) = total_minimal_button_presses(&machines) {
        println!(
            "The fewest button presses required to correctly configure all indicator lights is {n}"
        );
    } else {
        println!("At least one machine could not be configured with the given buttons");
    }

    Ok(())
}

fn total_minimal_button_presses(machines: &[Machine]) -> Option<u64> {
    machines.iter().map(minimal_button_presses).sum()
}

fn minimal_button_presses(machine: &Machine) -> Option<u64> {
    let mut visited: HashSet<u64> = HashSet::with_capacity(1024);
    let mut queue: VecDeque<(u64, u64)> = VecDeque::with_capacity(1024);
    visited.insert(0);
    queue.push_back((0, 0));
    while let Some((lights, n)) = queue.pop_front() {
        if lights == machine.light_diagram {
            return Some(n);
        }
        for button in &machine.buttons {
            let next = lights ^ button;
            if !visited.contains(&next) {
                queue.push_back((next, n + 1));
                visited.insert(next);
            }
        }
    }
    None
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Machine {
    light_diagram: u64,
    buttons: Vec<u64>,
}

fn parse(input: &str) -> Result<Vec<Machine>, String> {
    input.lines().map(parse_machine).collect()
}

fn parse_machine(line: &str) -> Result<Machine, String> {
    let (diagram, rest) = line
        .split_once("] ")
        .ok_or_else(|| format!("unable to find light diagram in '{line}"))?;

    let diagram = diagram
        .strip_prefix('[')
        .ok_or_else(|| format!("light diagram does not start with '[' in {line}"))?;
    if diagram.len() > 64 {
        return Err(format!(
            "we can only handle up to 64 lights per machine, was {}",
            diagram.len()
        ));
    }
    let light_diagram: u64 = diagram
        .chars()
        .fold(0u64, |d, c| (d << 1) | if c == '#' { 1 } else { 0 });

    let (buttons, _) = rest
        .split_once(" {")
        .ok_or_else(|| format!("unable to find voltage requirements in '{line}"))?;
    let buttons = buttons
        .strip_prefix('(')
        .and_then(|b| b.strip_suffix(')'))
        .ok_or_else(|| format!("buttons in '{line}' have unexpected parantheses"))?;
    let buttons: Vec<u64> = buttons
        .split(") (")
        .map(|button| {
            button.split(',').map(|t| {
                let index = t
                    .parse::<u8>()
                    .map_err(|e| format!("unable to parse button toggle index '{t}': {e}"))?;
                if index > 64 {
                    Err(format!(
                        "we can only handle up to 64 lights per machine, button index was {index}"
                    ))
                } else {
                    Ok(index)
                }
            }).try_fold(0, |button, i| {
                let i = i?;
                Ok::<u64, String>(button | (1<<(diagram.len()as u8 -1-i)))
            })
        })
        .collect::<Result<_, _>>()?;
    Ok(Machine {
        light_diagram,
        buttons,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

    #[test]
    fn minimal_button_presses_works_for_first_example_machine() {
        // given
        let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")
            .expect("expected valid input");

        // when
        let n = minimal_button_presses(&machine);

        // then
        assert_eq!(n, Some(2));
    }

    #[test]
    fn total_minimal_button_presses_works_for_example() {
        // given
        let machines = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let n = total_minimal_button_presses(&machines);

        // then
        assert_eq!(n, Some(7));
    }
}
