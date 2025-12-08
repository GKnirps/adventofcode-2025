#![forbid(unsafe_code)]

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let junction_boxes = parse(&content)?;

    let three_circ_product = three_largest_circuits(&junction_boxes, 1000);
    println!("the product of the size of the three largest circuits is {three_circ_product}");

    if let Some(all_connected_product) = connect_all(&junction_boxes) {
        println!(
            "The product of x coordinates of the last boxes I need to connect is {all_connected_product}"
        );
    } else {
        println!("I could not connect all boxes. This is a bug in the code.");
    }

    Ok(())
}

fn connect_all(junction_boxes: &[Pos]) -> Option<u64> {
    let distances = calc_distances(junction_boxes);
    // there are more efficient ways to do this, but this is simpler for now
    let mut circuits: Vec<HashSet<Pos>> = Vec::with_capacity(junction_boxes.len());
    for Connection {
        a,
        b,
        distance_sq: _,
    } in distances
    {
        let ai = circuits
            .iter()
            .enumerate()
            .find(|(_, c)| c.contains(&a))
            .map(|(i, _)| i);
        let bi = circuits
            .iter()
            .enumerate()
            .find(|(_, c)| c.contains(&b))
            .map(|(i, _)| i);
        match (ai, bi) {
            (Some(ai), Some(bi)) => {
                if let Ok([circ_a, circ_b]) = circuits.get_disjoint_mut([ai, bi]) {
                    circ_a.extend(circ_b.iter());
                    circuits.swap_remove(bi);
                }
            }
            (Some(ai), None) => {
                circuits[ai].insert(b);
            }
            (None, Some(bi)) => {
                circuits[bi].insert(a);
            }
            (None, None) => {
                circuits.push(HashSet::from([a, b]));
            }
        }
        if circuits.iter().any(|c| c.len() == junction_boxes.len()) {
            return Some(a.0 * b.0);
        }
    }
    None
}

fn three_largest_circuits(junction_boxes: &[Pos], n_pairs: usize) -> usize {
    let distances = calc_distances(junction_boxes);
    let mut edges: HashMap<Pos, Vec<Pos>> = HashMap::with_capacity(n_pairs * 2);

    for Connection {
        a,
        b,
        distance_sq: _,
    } in distances.iter().take(n_pairs)
    {
        edges.entry(*a).or_insert(Vec::with_capacity(10)).push(*b);
        edges.entry(*b).or_insert(Vec::with_capacity(10)).push(*a);
    }

    let mut visited: HashSet<Pos> = HashSet::with_capacity(junction_boxes.len());
    let mut circuit_sizes: BinaryHeap<usize> = BinaryHeap::with_capacity(junction_boxes.len());
    let mut stack: Vec<Pos> = Vec::with_capacity(junction_boxes.len());
    for junction in junction_boxes {
        if visited.contains(junction) {
            continue;
        }
        let mut count: usize = 0;
        stack.clear();
        stack.push(*junction);
        while let Some(j1) = stack.pop() {
            if visited.contains(&j1) {
                continue;
            }
            count += 1;
            visited.insert(j1);
            if let Some(j2) = edges.get(&j1) {
                stack.extend(j2);
            }
        }
        circuit_sizes.push(count);
    }

    circuit_sizes.pop().unwrap_or(1)
        * circuit_sizes.pop().unwrap_or(1)
        * circuit_sizes.pop().unwrap_or(1)
}

#[derive(Copy, Clone, Debug)]
struct Connection {
    a: Pos,
    b: Pos,
    distance_sq: u64,
}

// screw this, I have enough memory to store a million shortest connections
fn calc_distances(boxes: &[Pos]) -> Vec<Connection> {
    let mut connections: Vec<Connection> =
        Vec::with_capacity((boxes.len() * (boxes.len() + 1)) / 2);
    for (i, box1) in boxes.iter().enumerate() {
        for box2 in &boxes[i + 1..] {
            let distance = distance_sq(*box1, *box2);
            connections.push(Connection {
                a: *box1,
                b: *box2,
                distance_sq: distance,
            });
        }
    }
    connections.sort_unstable_by_key(|conn| conn.distance_sq);
    connections
}

fn distance_sq((x1, y1, z1): Pos, (x2, y2, z2): Pos) -> u64 {
    (x1.max(x2) - x1.min(x2)).pow(2)
        + (y1.max(y2) - y1.min(y2)).pow(2)
        + (z1.max(z2) - z1.min(z2)).pow(2)
}

type Pos = (u64, u64, u64);

fn parse(input: &str) -> Result<Vec<Pos>, String> {
    input.lines().map(parse_pos).collect()
}

fn parse_pos(line: &str) -> Result<Pos, String> {
    let (first, rest) = line
        .split_once(',')
        .ok_or_else(|| format!("unable to split '{line}'"))?;
    let (second, third) = rest
        .split_once(',')
        .ok_or_else(|| format!("unable to split '{rest}'"))?;
    let x: u64 = first
        .parse()
        .map_err(|e| format!("unable to parse '{first}': {e}"))?;
    let y: u64 = second
        .parse()
        .map_err(|e| format!("unable to parse '{second}': {e}"))?;
    let z: u64 = third
        .parse()
        .map_err(|e| format!("unable to parse '{third}': {e}"))?;

    Ok((x, y, z))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn three_largest_circuits_works_for_example() {
        // given
        let boxes = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let result = three_largest_circuits(&boxes, 10);

        // then
        assert_eq!(result, 40);
    }

    #[test]
    fn connect_all_works_for_example() {
        // given
        let boxes = parse(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let result = connect_all(&boxes);

        // then
        assert_eq!(result, Some(25272));
    }
}
