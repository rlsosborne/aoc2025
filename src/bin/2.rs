use aoc2025::util::read_lines;
use aoc2025::util::get_input_path;

fn repeated_n(s: &str, n: usize) -> bool {
    let len = s.len();
    if (len % n) != 0 {
        return false;
    }
    let slice_len = len / n;
    return (0..n).map(|i| &s[(i * slice_len)..((i + 1) * slice_len)])
        .all(|part| part == &s[0..slice_len]);
}

fn invalid(id: u64) -> bool {
    let s = id.to_string();
    let len = s.len();
    return (2..len+1).map(|n| repeated_n(&s, n)).any(|b| b);
}

fn ids_in_range(range: &str) -> std::ops::Range<u64> {
    let bounds = range.split("-").map(|id| id.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    return bounds[0]..bounds[1]
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let input = lines.join("");
    let ids = input.split(",").flat_map(|range| ids_in_range(range));
    let repeated_twice_sum = ids.clone().filter(|id| repeated_n(&id.to_string(), 2)).sum::<u64>();
    let invalid_sum = ids.filter(|id| invalid(*id)).sum::<u64>();
    println!("Sum of repeated twice IDs: {}", repeated_twice_sum);
    println!("Sum of invalid IDs: {}", invalid_sum);
}
