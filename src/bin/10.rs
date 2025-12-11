use aoc2025::util::{get_input_path, read_lines};
use std::collections::HashSet;

struct Problem {
    light_goal: Vec::<bool>,
    joltage_goal: Vec::<u32>,
    buttons: Vec::<Vec::<usize>>
}

fn parse_button(button: &str) -> Vec::<usize> {
   button[1..button.len()-1].split(',').map(|n| n.parse::<usize>().unwrap()).collect()
}

fn parse_problem(line: &String) -> Problem {
    let Some((light_goal, line)) = line.split_once(']') else { panic!(); };
    let light_goal = &light_goal[1..];
    let light_goal: Vec::<bool> = light_goal.chars().map(|c| c == '#').collect();
    let Some((buttons, joltage_goal)) = line.split_once('{')  else { panic!(); };
    let buttons = buttons.trim().split_whitespace().map(|s| parse_button(s)).collect();
    let joltage_goal = &joltage_goal[..joltage_goal.len()-1];
    let joltage_goal: Vec::<u32> = joltage_goal.split(',').map(|n| n.parse::<u32>().unwrap()).collect();
    Problem{light_goal:light_goal, joltage_goal:joltage_goal, buttons:buttons}
}

fn toggle(state: Vec::<bool>, indices: &Vec::<usize>) -> Vec::<bool> {
    let mut state = state;
    for i in indices {
        state[*i] = !state[*i];
    }
    state
}

fn solve_lights(problem: &Problem) -> usize {
    let initial = vec![
        vec![false; problem.light_goal.len()]
    ];
    let all_combinations = (0..).scan((initial, HashSet::<Vec::<bool>>::new()), |acc, _| {
        let (prev, seen) = acc;
        let next: Vec::<Vec::<bool>> = prev.into_iter().flat_map(|pattern| {
            problem.buttons.iter().map(move |button| {
                toggle(pattern.clone(), button)
            })
        }).flat_map(|pattern| seen.insert(pattern.clone()).then_some(pattern)).collect();
        *acc = (next.clone(), std::mem::take(seen));
        Some(next)
    });
    for (iterations, combinations) in all_combinations.enumerate() {
        if combinations.is_empty() {
            break;
        }
        for combination in combinations {
            if combination == problem.light_goal {
                return iterations + 1
            }
        }
    }
    panic!()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let problems = lines.iter().map(parse_problem);
    let light_solutions = problems.clone().map(|problem| solve_lights(&problem));
    let light_total: usize = light_solutions.sum();
    println!("Light total: {}", light_total);
}
