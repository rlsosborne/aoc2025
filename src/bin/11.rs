use aoc2025::util::{get_input_path, read_lines};
use std::collections::HashMap;

#[derive(Debug, Default)]
struct Graph {
    names: HashMap::<String, usize>,
    children: Vec::<Vec::<usize>>
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn get_index(&mut self, name: &str) -> usize {
        self.names.get(name).map(|i| *i).map_or_else(|| {
            let i = self.names.len();
            self.names.insert(name.to_string(), i);
            self.children.push(Default::default());
            i
        }, |i| i)
    }
    fn parse_line(&mut self, line: &str) {
        let Some((src, line)) = line.split_once(':') else { panic!(); };
        let src = self.get_index(src);
        let dsts = line.trim().split_whitespace().map(|dst| self.get_index(dst));
        self.children[src] = dsts.collect();
    }
}

fn count_paths(graph: &Graph, src: usize, dst: usize, cache: &mut HashMap::<(usize, usize), u64>) -> u64 {
    if src == dst {
        return 1;
    }
    if let Some(&num_paths) = cache.get(&(src, dst)) {
        return num_paths;
    }
    let num_paths = graph.children[src].iter().map(|&child| count_paths(graph, child, dst, cache)).sum();
    cache.insert((src, dst), num_paths);
    num_paths
}

fn count_paths_by_name(graph: &Graph, src: &str, dst: &str, cache: &mut HashMap::<(usize, usize), u64>) -> u64 {
    let src = graph.names.get(src);
    let dst = graph.names.get(dst);
    match (src, dst) {
        (Some(&src), Some(&dst)) => count_paths(graph, src, dst, cache),
        _ => 0
    }
}

fn main() {
    let mut graph: Graph = Graph::new();
    let lines = read_lines(get_input_path().join("input.txt"));
    for line in lines {
        graph.parse_line(&line);
    }
    let graph = graph;
    let mut cache: HashMap::<(usize, usize), u64> = Default::default();
    let num_paths_out = count_paths_by_name(&graph, "you", "out", &mut cache);
    println!("Num paths out: {}", num_paths_out);
    let routes = [
        ["svr", "dac", "fft", "out"],
        ["svr", "fft", "dac", "out"],
    ];
    let num_problematic_paths: u64 = routes.iter().map(|route| {
        route.windows(2).map(|w| count_paths_by_name(&graph, w[0], w[1], &mut cache)).product::<u64>()
    }).sum();
    println!("Num problematic paths: {}", num_problematic_paths);
}
