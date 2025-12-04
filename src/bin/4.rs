use aoc2025::util::{cartestian_product, get_input_path, read_lines};
use std::cmp::min;
use std::collections::HashSet;

struct Grid {
    cells: Vec<Vec<bool>>
}

impl Grid {
    fn height(&self) -> usize {
        self.cells.len()
    }
    fn width(&self) -> usize {
        self.cells[0].len()
    }
    fn coords(&self) -> impl Iterator<Item=(usize, usize)> {
        cartestian_product(0..self.height(), 0..self.width())
    }
    fn rolls(&self) -> impl Iterator<Item=(usize, usize)> {
        self.coords().filter(|&coord| self.at(coord))
    }
    fn neighbours(&self, coord: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        let (r, c) = coord;
        let linear_neighbours = |i, size| i - min(i, 1)..min(i + 2, size);
        let rows = linear_neighbours(r, self.height()); 
        let cols = linear_neighbours(c, self.width());
        cartestian_product(rows, cols).filter(move |&neighbour| neighbour != coord)    
    }
    fn neighbouring_rolls(&self, coord: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
        self.neighbours(coord).filter(|&coord| self.at(coord))
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        let (r, c) = coord;
        self.cells[r][c]
    }
    fn remove_roll(&mut self, coord: (usize, usize)) {
        let (r, c) = coord;
        self.cells[r][c] = false;
    }
    fn num_rolls(&self) -> usize {
        self.rolls().count()
    }
}

fn is_accessible(grid: &Grid, coord: (usize, usize)) -> bool {
    grid.neighbouring_rolls(coord).count() < 4
}

fn remove_accessible(grid: &mut Grid) {
    let mut worklist = HashSet::new();
    worklist.extend(grid.rolls());
    while !worklist.is_empty() {
        let coord = *worklist.iter().next().unwrap();
        worklist.remove(&coord);
        if is_accessible(grid, coord) {
            grid.remove_roll(coord);
            worklist.extend(grid.neighbouring_rolls(coord));
        }
    }
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let mut grid = Grid{cells: lines.iter().map(|line| line.chars().map(|c| c == '@').collect()).collect()};
    let accessible_count = grid.rolls().filter(|coord| is_accessible(&grid, *coord)).count();
    println!("Number of accessible rolls: {}", accessible_count);
    let starting_rolls = grid.num_rolls();
    remove_accessible(&mut grid);
    let ending_rolls = grid.num_rolls();
    println!("Removable rolls: {}", starting_rolls - ending_rolls);
}
