use aoc2025::util::{get_input_path, read_lines};

struct JunctionBoxes {
    circuits: Vec<Vec<usize>>,
    box_to_circuit: Vec<usize>
}

impl JunctionBoxes {
    fn new(n: usize) -> Self {
        JunctionBoxes {
            circuits: (0..n).map(|i| vec!(i)).collect(),
            box_to_circuit: (0..n).collect()
        }
    }

    fn connect(&mut self, i: usize, j: usize) {
        let i_circuit= self.box_to_circuit[i];
        let j_circuit= self.box_to_circuit[j];
        if i_circuit != j_circuit {
            self.circuits[j_circuit].iter().for_each(|&junction| {
                self.box_to_circuit[junction] = i_circuit;
            });
            let tmp = std::mem::take(&mut self.circuits[j_circuit]);
            self.circuits[i_circuit].extend(tmp);
        }
    }
}

fn part1(distances: &Vec<(usize, usize, f32)>, n: usize, num_connections: usize) -> usize {
    let mut boxes = JunctionBoxes::new(n);
    let direct_connections = distances.iter().take(num_connections).map(|&(i, j, _)| (i, j));
    direct_connections.for_each(|(i, j)| {
        boxes.connect(i, j);
    });
    let mut circuit_sizes: Vec<usize> = boxes.circuits.iter().map(|acc| acc.len()).collect();
    circuit_sizes.sort();
    circuit_sizes[n-3..].iter().product()
}

fn all_same(v: &Vec<usize>) -> bool {
    v.windows(2).all(|w| w[0] == w[1])
}

fn part2(coords: &Vec<[f32; 3]>, distances: &Vec<(usize, usize, f32)>, n: usize) -> u64 {
    let mut boxes = JunctionBoxes::new(n);
    let mut it = distances.iter();
    let mut last: Option<(usize, usize)> = None;
    while !all_same(&boxes.box_to_circuit) {
        let (i, j, _) = it.next().unwrap();
        boxes.connect(*i, *j);
        last = Some((*i, *j))
    }
    let (i, j) = last.unwrap();
    coords[i][0] as u64 * coords[j][0] as u64
}

fn distance(a: [f32; 3], b:[f32; 3]) -> f32 {
    a.iter().zip(b).map(|(x1, x2)| (x2 - x1) * (x2 - x1)).sum::<f32>().sqrt()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let num_connections = 1000;
    let coords: Vec<[f32; 3]> = lines.iter().map(|line| {
        let mut parts = line.split(",").map(|s| s.parse().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        [x, y, z]
    }).collect();
    let n = coords.len();
    let pairs = (0..n).flat_map(|j| (0..j).map(move |i| (i, j)));
    let mut distances: Vec<(usize, usize, f32)> = pairs.map(|(i, j)| {
        (i, j, distance(coords[i], coords[j]))
    }).collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!("Part 1 answer: {}", part1(&distances, n, num_connections));
    println!("Part 2 answer: {}", part2(&coords, &distances, n));
}
