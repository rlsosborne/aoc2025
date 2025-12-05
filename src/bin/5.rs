use aoc2025::util::{get_input_path, read_lines};

fn simplify(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.sort();
    let mut simplified: Vec<(u64, u64)> = Vec::new();
    for (low, high) in ranges {
        if let Some((_, last_high)) = simplified.last_mut() {
            if low <= *last_high {
                *last_high = (*last_high).max(high);
                continue;
            }
        }
        simplified.push((low, high));
    }
    simplified
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let sep = lines.iter().position(|line| line.trim().is_empty()).unwrap();
    let ranges = lines[..sep].iter().map(|line| {
        let bounds: Vec<u64> = line.split("-").map(|bound| bound.parse::<u64>().unwrap()).collect();
        (bounds[0], bounds[1])
    }).collect::<Vec<(u64, u64)>>();
    let ranges = simplify(ranges);
    let ids = lines[sep+1..].iter().map(|line| {
        line.parse::<u64>().unwrap()
    });
    let num_valid = ids.filter(|id| {
        ranges.iter().any(|(low, high)| id >= low && id <= high)
    }).count();
    println!("Number of valid IDs: {}", num_valid);
    let num_valid_in_ranges: u64 = ranges.iter().map(|(low, high)| high - low + 1).sum();
    println!("Number of valid ID in ranges: {}", num_valid_in_ranges);
}
