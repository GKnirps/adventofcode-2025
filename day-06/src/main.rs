#![forbid(unsafe_code)]

use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(Path::new(&filename)).map_err(|e| e.to_string())?;
    let (number_lines, op_line) = parse(&content)?;

    let total = grand_total(&number_lines, &op_line);
    println!("The grand total of all operations is {total}");

    let (ceph_numbers, op_line) = parse_cephalopod_numbers(&content)?;
    let total = grand_cephalopod_total(&ceph_numbers, &op_line);
    println!("The grand total of cephalopod numbers os {total}");

    Ok(())
}

fn grand_cephalopod_total(numbers: &[Vec<u64>], ops: &[Operator]) -> u64 {
    numbers
        .iter()
        .zip(ops)
        .map(|(n, op)| match op {
            Operator::Add => n.iter().sum::<u64>(),
            Operator::Mul => n.iter().product::<u64>(),
        })
        .sum()
}

fn grand_total(numbers: &[Vec<u64>], ops: &[Operator]) -> u64 {
    ops.iter()
        .enumerate()
        .map(|(i, op)| match op {
            Operator::Add => numbers.iter().map(|line| line[i]).sum::<u64>(),
            Operator::Mul => numbers.iter().map(|line| line[i]).product::<u64>(),
        })
        .sum()
}

fn parse(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>), String> {
    let number_lines: Vec<Vec<u64>> = input
        .lines()
        .filter(|line| !line.starts_with('+') && !line.starts_with('*'))
        .map(parse_number_line)
        .collect::<Result<_, _>>()?;
    let op_line: Vec<Operator> = input
        .lines()
        .find(|line| line.starts_with('+') || line.starts_with('*'))
        .map(parse_op_line)
        .unwrap_or_else(|| Err("missing operator line".to_string()))?;

    if number_lines.is_empty() {
        Err("missing number lines".to_string())
    } else if number_lines.iter().any(|line| line.len() != op_line.len()) {
        Err("nor all lines have the same length".to_string())
    } else {
        Ok((number_lines, op_line))
    }
}

fn parse_number_line(line: &str) -> Result<Vec<u64>, String> {
    line.split_whitespace()
        .map(|s| {
            s.parse::<u64>()
                .map_err(|e| format!("unable to parse '{s}': {e}"))
        })
        .collect()
}

fn parse_cephalopod_numbers(input: &str) -> Result<(Vec<Vec<u64>>, Vec<Operator>), String> {
    if !input.is_ascii() {
        return Err("expected ASCII only string".to_string());
    }
    let n_rows = input.lines().count() - 1;
    let n_cols = input
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(|| "empty input!".to_string())?;
    if input.lines().any(|line| line.len() != n_cols) {
        return Err("not all lines have the same length".to_string());
    }

    let operators: Vec<Operator> = parse_op_line(
        input
            .lines()
            .nth(n_rows)
            .ok_or_else(|| "missing operator row".to_string())?,
    )?;
    let mut numbers: Vec<Vec<u64>> = Vec::with_capacity(n_cols);
    let mut number_group: Vec<u64> = Vec::with_capacity(n_cols);
    for col in 0..n_cols {
        // we checked for ascii-only at the beginning, so we can just index the string here
        if (0..n_rows).all(|row| {
            input[col + row * (n_cols + 1)..=col + row * (n_cols + 1)]
                .trim()
                .is_empty()
        }) {
            numbers.push(number_group);
            number_group = Vec::with_capacity(n_cols);
        } else {
            number_group.push(
                (0..n_rows)
                    .filter_map(|row| {
                        let r = &input[col + row * (n_cols + 1)..=col + row * (n_cols + 1)];
                        if r.trim().is_empty() {
                            None
                        } else {
                            Some(
                                r.parse::<u64>()
                                    .map_err(|e| format!("unable to parse: {e}")),
                            )
                        }
                    })
                    .try_fold(0u64, |n, d| -> Result<u64, String> { Ok(n * 10 + d?) })?,
            );
        }
    }
    numbers.push(number_group);
    Ok((numbers, operators))
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Operator {
    Add,
    Mul,
}

fn parse_op_line(line: &str) -> Result<Vec<Operator>, String> {
    line.split_whitespace()
        .map(|s| match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mul),
            _ => Err(format!("unknown operator: '{s}'")),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str =
        "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

    #[test]
    fn parse_cephalopod_numbers_works_for_example() {
        // when
        let result = parse_cephalopod_numbers(EXAMPLE_INPUT);

        // then
        let (numbers, ops) = result.expect("expected successful parsing");
        assert_eq!(
            ops,
            &[Operator::Mul, Operator::Add, Operator::Mul, Operator::Add]
        );
        assert_eq!(
            numbers,
            &[
                vec![1, 24, 356],
                vec![369, 248, 8],
                vec![32, 581, 175],
                vec![623, 431, 4],
            ]
        );
    }

    #[test]
    fn grand_cephalopod_total_works_for_example() {
        // given
        let (numbers, ops) = parse_cephalopod_numbers(EXAMPLE_INPUT).expect("expected valid input");

        // when
        let result = grand_cephalopod_total(&numbers, &ops);

        // then
        assert_eq!(result, 3263827);
    }
}
