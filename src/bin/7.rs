use aoc2025::util::{get_input_path, read_lines};

fn count_splits(lines: &Vec<String>) -> u64 {
    let beams : Vec<bool> = lines[0].chars().map(|c| c == 'S').collect();
    let (splits, _) =
        lines[1..].iter().fold((0, beams), |acc, line| {
            let (mut total, beams) = acc;
            let splits : Vec<bool> = line.chars().zip(beams.iter()).map(|(c, &b)| {
                b && c == '^'
            }).collect();
            total += splits.iter().copied().filter(|&b| b).count() as u64;
            let next: Vec<bool> = (0..line.len()).map(|i| {
                beams[i] && !splits[i] ||
                (i > 0 && splits[i-1]) ||
                (i + 1 < splits.len() && splits[i+1])
            }).collect();
            (total, next)
        });
    splits
}

fn count_timelines(lines: &Vec<String>) -> u64 {
    let beams : Vec<u64> = lines[0].chars().map(|c| (c == 'S') as u64).collect();
    let timelines =
        lines[1..].iter().fold(beams, |beams, line| {
            let splits : Vec<u64> = line.chars().zip(beams.iter()).map(|(c, &t)| {
                (c == '^').then(||t).unwrap_or(0)
            }).collect();
            (0..line.len()).map(|i| {
                (splits[i] == 0).then(||beams[i]).unwrap_or(0) +
                (i > 0).then(||splits[i-1]).unwrap_or(0) +
                (i + 1 < splits.len()).then(||splits[i+1]).unwrap_or(0)
            }).collect()
        });
    timelines.iter().sum()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    println!("Number of splits: {}", count_splits(&lines));
    println!("Number of timelines: {}", count_timelines(&lines));
}
