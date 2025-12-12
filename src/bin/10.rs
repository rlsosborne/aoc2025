use aoc2025::util::{get_input_path, read_lines};
use std::collections::HashSet;

#[derive(Debug)]
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

#[derive(PartialEq)]
enum PropagateResult {
    NoChange,
    Change,
    Unsat
}

impl std::ops::BitOr for PropagateResult {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PropagateResult::Unsat, _) => PropagateResult::Unsat,
            (_, PropagateResult::Unsat) => PropagateResult::Unsat,
            (PropagateResult::NoChange, PropagateResult::NoChange) => PropagateResult::NoChange,
            _ => PropagateResult::Change
        }
    }
}

#[derive(Debug, Clone)]
struct Domain {
    low: i32,
    high: i32
}

impl Domain {
    fn update_low(&mut self, value: i32) -> PropagateResult {
        if value > self.high {
            PropagateResult::Unsat
        } else if value > self.low {
            self.low = value;
            PropagateResult::Change
        } else {
            PropagateResult::NoChange
        }
    }
    fn update_high(&mut self, value: i32) -> PropagateResult {
        if value < self.low {
            PropagateResult::Unsat
        } else if value < self.high {
            self.high = value;
            PropagateResult::Change
        } else {
            PropagateResult::NoChange
        }
    }
}

trait Constrait {
    fn propagate(&self, domains: &mut Vec::<Domain>) -> PropagateResult;
}

struct SumConstraint {
    vars: Vec::<usize>,
    dst: usize
}

impl Constrait for SumConstraint {
    fn propagate(&self, domains: &mut Vec<Domain>) -> PropagateResult {
        let sum_low = self.vars.iter().map(|&v| domains[v].low).sum();
        let sum_high = self.vars.iter().map(|&v| domains[v].high).sum();
        let result = domains[self.dst].update_low(sum_low);
        let result = result | domains[self.dst].update_high(sum_high);
        let dst_low = domains[self.dst].low;
        let dst_high = domains[self.dst].high;
        let result = self.vars.iter().fold(result, |result, &v| {
            let new_low = dst_low - sum_high + domains[v].high;
            let new_high = dst_high - sum_low + domains[v].low;
            result | domains[v].update_low(new_low) | domains[v].update_high(new_high)
        });
        result
    }
}

fn solve(mut domains: Vec<Domain>, constraints: &Vec<Box<dyn Constrait>>) -> Option<Vec<Domain>> {
    loop {
        let mut result = PropagateResult::NoChange;
        for constraint in constraints {
            result = result | constraint.propagate(&mut domains);
            if result == PropagateResult::Unsat {
                return None;
            }
        }
        if result == PropagateResult::NoChange {
            break;
        }
    }
    if let Some((i, _)) = domains.iter().enumerate().filter(|(_, domain)| {
        domain.low != domain.high
    }).min_by_key(|(_, domain)| {
        domain.high - domain.low
    }) {
        for value in (domains[i].low..domains[i].high+1).rev() {
            let mut new_domains = domains.clone();
            new_domains[i].low = value;
            new_domains[i].high = value;
            if let Some(solution) = solve(new_domains, constraints) {
                return Some(solution);
            }
        }
        return None
    } else {
        return Some(domains)
    }
}

#[derive(Clone, Copy)]
struct Var {
    index: usize
}

#[derive(Default)]
struct Solver {
    domains: Vec<Domain>,
    constraints: Vec<Box<dyn Constrait>>
}

impl Solver {
    fn add_var(&mut self, low: i32, high: i32) -> Var {
        let index = self.domains.len();
        self.domains.push(Domain{low:low, high:high});
        Var{index}
    }
    fn sum_eq(&mut self, vars: &[Var], dst: Var) {
        let vars: Vec<usize> = vars.iter().map(|Var{ index: v }| *v).collect();
        let Var { index: dst} = dst;
        self.constraints.push(Box::new(SumConstraint{vars:vars, dst:dst}));
    }
    fn minimise(&self, var: Var) -> Option<i32> {
        let mut best = None;
        loop {
            let mut new_domains = self.domains.clone();
            if let Some(best) = best {
                if new_domains[var.index].update_high(best - 1) == PropagateResult::Unsat {
                    break;
                }
            }
            let Some(solution) = solve(new_domains, &self.constraints) else { break; };
            best = Some(solution[var.index].low);
            println!("best: {}", solution[var.index].low);
        }
        best
    }
}

fn solve_joltage(problem: &Problem) -> usize {
    println!("problem: {:?}", problem);
    let mut solver = Solver::default();
    let num_buttons = problem.buttons.len();
    let num_counters = problem.joltage_goal.len();
    let mut counters_to_buttons = vec!(Vec::<usize>::default();num_counters);
    for (i, counters) in problem.buttons.iter().enumerate() {
        for &counter in counters {
            counters_to_buttons[counter].push(i)
        }
    }
    let vars: Vec<Var> = (0..num_buttons).map(|_| {
        solver.add_var(0, 100000)
    }).collect();
    for (&goal, buttons) in problem.joltage_goal.iter().zip(counters_to_buttons) {
        let sum = solver.add_var(goal as i32, goal as i32);
        let vars: Vec<Var> = buttons.iter().map(|&i| vars[i]).collect();
        solver.sum_eq(&vars, sum);
    }
    let sum = solver.add_var(0, 100000);
    solver.sum_eq(&vars, sum);
    solver.minimise(sum).unwrap() as usize
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let problems = lines.iter().map(parse_problem);
    let light_solutions = problems.clone().map(|problem| solve_lights(&problem));
    let light_total: usize = light_solutions.sum();
    println!("Light total: {}", light_total);
    // This took 25m31.208s on my 2020 Macbook Pro!
    let joltage_solutions = problems.map(|problem| solve_joltage(&problem));
    let joltage_total: usize = joltage_solutions.sum();
    println!("Joltage total: {}", joltage_total);
}
