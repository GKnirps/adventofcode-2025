#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let edges = parse(&content)?;

    let n_paths = paths_from_you_to_out(&edges)?;
    println!("There are {n_paths} paths from 'you' to 'out'");

    Ok(())
}

fn paths_from_you_to_out(edges: &HashMap<&str, Box<[&str]>>) -> Result<u64, String> {
    let sorted_vertices = sort_vertices(edges)?;
    let mut paths: HashMap<&str, u64> = HashMap::with_capacity(sorted_vertices.len());
    paths.insert("you", 1);
    for v in sorted_vertices {
        let n = paths.get(v).copied().unwrap_or(0);
        for to in edges.get(v).iter().flat_map(|v| v.iter()) {
            *paths.entry(to).or_insert(0) += n;
        }
    }
    Ok(paths.get("out").copied().unwrap_or(0))
}

fn sort_vertices<'v>(edges: &HashMap<&'v str, Box<[&'v str]>>) -> Result<Vec<&'v str>, String> {
    // first: make sure we only consider vertices that can be reached from "you"
    let mut vertices: HashSet<&str> = HashSet::with_capacity(edges.len() + 1);
    let mut stack: Vec<&str> = Vec::with_capacity(edges.len());
    stack.push("you");
    while let Some(v) = stack.pop() {
        if vertices.contains(v) {
            continue;
        }
        vertices.insert(v);
        for next_v in edges.get(v).iter().flat_map(|to| to.iter()) {
            if *next_v == "you" {
                return Err("there is a loop to 'you'".to_string());
            }
            stack.push(next_v);
        }
    }
    let vertices = vertices;

    let mut incoming_count: HashMap<&str, usize> = HashMap::with_capacity(edges.len());
    incoming_count.insert("you", 0);
    for to in edges
        .iter()
        .filter(|(v, _)| vertices.contains(*v))
        .flat_map(|(_, v)| v.iter())
    {
        *incoming_count.entry(to).or_insert(0) += 1;
    }

    let mut sorted: Vec<&str> = Vec::with_capacity(incoming_count.len() + 1);

    while !incoming_count.is_empty() {
        if let Some(predecessor) = incoming_count
            .iter()
            .find(|(_, n)| **n == 0)
            .map(|(v, _)| *v)
        {
            sorted.push(predecessor);
            for to in edges.get(predecessor).iter().flat_map(|v| v.iter()) {
                if let Some(count) = incoming_count.get_mut(to) {
                    *count -= 1;
                }
            }
            incoming_count.remove(predecessor);
        } else {
            return Err("there is a cycle in the graph, this won't work".to_string());
        }
    }
    if sorted.first() != Some(&"you") {
        Err("someting went wrong during sorting".to_string())
    } else {
        Ok(sorted)
    }
}

fn parse(input: &str) -> Result<HashMap<&str, Box<[&str]>>, String> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line
                .split_once(": ")
                .ok_or_else(|| format!("unable to split line '{line}'"))?;
            let to: Box<[&str]> = to.split_whitespace().collect();
            Ok((from, to))
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"#;

    #[test]
    fn sort_vertices_works_for_example() {
        // given
        let edges = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let result = sort_vertices(&edges);

        // then
        let vertices = result.expect("expected successful sorting");
        assert_eq!(vertices.len(), 8);
        assert_eq!(vertices.get(0), Some(&"you"));
        assert_eq!(vertices.get(7), Some(&"out"));
    }

    #[test]
    fn paths_from_you_to_out_works_for_example() {
        // given
        let edges = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let n = paths_from_you_to_out(&edges);

        // then
        assert_eq!(n, Ok(5));
    }
}
