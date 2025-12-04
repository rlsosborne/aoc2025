use aoc2025::util::read_lines;
use aoc2025::util::get_input_path;

fn sum_of_topk_in_seq(digits: &str, k: usize) -> u64 {
    let numbers: Vec<u32> = digits.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let n = numbers.len();
    let top = (n-k..n).scan(0, |acc, last| {
        let first = *acc;
        let indices = first..last+1;
        // If there are multiple highest max_by_key takes the last, so reverse the range to get the first.
        let highest_idx = indices.rev().max_by_key(|i| numbers[*i as usize]).unwrap();
        *acc = highest_idx + 1;
        Some(numbers[highest_idx])
    });
    top.fold(0, |acc, i| acc * 10 + i as u64)
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let max_joltage_2: u64 = lines.iter().map(|line| sum_of_topk_in_seq(&line, 2)).sum();
    let max_joltage_12: u64 = lines.iter().map(|line| sum_of_topk_in_seq(&line, 12)).sum();
    println!("Max joltage with 2 batteries: {}", max_joltage_2);
    println!("Max joltage with 12 batteries: {}", max_joltage_12);
}
