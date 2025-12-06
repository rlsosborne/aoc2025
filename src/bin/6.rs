use aoc2025::util::{get_input_path, read_lines};
use std::iter;

fn read_human_numbers(lines: &[String]) -> Vec<Vec<u64>> {
    let num_cols = lines[0].split_whitespace().count();
    let mut cols: Vec<Vec<u64>> = (0..num_cols).map(|_| Vec::new()).collect();
    lines.iter().map(|line| line.split_whitespace()).for_each(|fields| {
        cols.iter_mut().zip(fields).for_each(|(col, field)| {
            col.push(field.parse::<u64>().unwrap());
        });
    });
    cols
}

fn read_cephalopod_numbers(lines: &[String]) -> Vec<Vec<u64>> {
    let mut cols = vec![0; lines[0].len()];
    lines.iter().for_each(|line| cols.iter_mut().zip(line.chars()).for_each(|(col, c)| {
        c.to_digit(10).into_iter().for_each(|digit| *col = *col * 10 + digit as u64)
    }));
    let empty_cols: Vec<usize> = cols.iter().enumerate().filter(|(_, col)| **col == 0).map(|(i, _)| i).collect();
    let starts = iter::once(0usize).chain(empty_cols.iter().map(|i| i + 1));
    let ends = empty_cols.iter().cloned().chain(iter::once(cols.len()));
    starts.zip(ends).map(|(start, end)| {
        cols[start..end].to_vec()
    }).collect()
}

fn apply_operators(numbers: &Vec<Vec<u64>>, operators: &Vec<fn(u64, u64) -> u64>) -> u64 {
    numbers.iter().zip(operators.iter()).map(|(col, op)| {
        col.iter().cloned().reduce(op).unwrap()
    }).sum()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let operators = lines.last().unwrap().split_whitespace().map(|field: &str| match field {
        "*" => |a: u64, b: u64| a * b,
        "+" => |a: u64, b: u64| a + b,
        _ => panic!("Unknown operator {}", field)
    }).collect::<Vec<fn(u64, u64) -> u64>>();
    let human_numbers = read_human_numbers(&lines[0..lines.len()-1]);
    let human_total = apply_operators(&human_numbers, &operators);
    println!("Human total: {}", human_total);
    let cephalopod_numbers = read_cephalopod_numbers(&lines[0..lines.len()-1]);
    let cephalopod_total = apply_operators(&cephalopod_numbers, &operators);
    println!("Cephalopod_total total: {}", cephalopod_total);
}
