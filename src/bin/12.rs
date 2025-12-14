use aoc2025::util::{cartestian_product, get_input_path, read_lines};

trait ShapeView {
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn at(&self, coord: (usize, usize)) -> bool;
    fn coords(&self) -> impl Iterator<Item=(usize, usize)> {
        cartestian_product(0..self.width(), 0..self.height()).filter(|&coord| self.at(coord))
    }
    fn area(&self) -> usize {
        self.coords().count()
    }
    fn rotate(&self, turns: usize) -> RotatedShape<'_, Self>{
        RotatedShape { shape: self, turns }
    }
    fn translate(&self, offset: (usize, usize)) -> TranslatedShape<'_, Self>{
        TranslatedShape { shape: self, offset }
    }
    fn flip(&self) -> FlippedShape<'_, Self>{
        FlippedShape { shape: self }
    }
    fn as_shape(&self) -> Shape {
        let mut cells = vec![vec![false; self.height()];self.width()];
        for (x, y) in self.coords() {
            cells[x][y] = true;
        }
        Shape{cells}
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Shape {
    cells: Vec<Vec<bool>>
}

impl ShapeView for Shape {
    fn width(&self) -> usize {
        self.cells.len()
    }
    fn height(&self) -> usize {
        self.cells[0].len()
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        self.cells[x][y]
    }
}

struct RotatedShape<'a, T: ShapeView + ?Sized> {
    shape: &'a T,
    turns: usize
}

impl<'a, T: ShapeView + ?Sized> ShapeView for RotatedShape<'a, T> {
    fn height(&self) -> usize {
        if self.turns % 2 == 0 { self.shape.height() } else { self.shape.width() }
    }
    fn width(&self) -> usize {
        if self.turns % 2 == 0 { self.shape.width() } else { self.shape.height() }
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        let map_coord = |coord: (usize, usize)| {
            let (x, y) = coord;
            match self.turns % 4 {
                0 => (x, y),
                1 => (self.shape.width() - 1 - y, x),
                2 => (self.shape.width() - 1 - x, self.shape.height() - 1 - y),
                3 => (y, self.shape.height() - 1 - x),
                _ => panic!()
            }
        };
        self.shape.at(map_coord(coord))
    }
}

struct FlippedShape<'a, T: ShapeView + ?Sized> {
    shape: &'a T
}

impl<'a, T: ShapeView + ?Sized> ShapeView for FlippedShape<'a, T> {
    fn height(&self) -> usize {
        self.shape.height()
    }
    fn width(&self) -> usize {
        self.shape.width()
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        self.shape.at((self.shape.width() - 1 - x, y))
    }
}

struct TranslatedShape<'a, T: ShapeView + ?Sized> {
    shape: &'a T,
    offset: (usize, usize)
}

impl<'a, T: ShapeView + ?Sized> ShapeView for TranslatedShape<'a, T> {
    fn height(&self) -> usize {
        self.shape.height()
    }
    fn width(&self) -> usize {
        self.shape.width()
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        if coord.0 < self.offset.0 || coord.1 < self.offset.1 {
            false
        } else {
            self.shape.at((coord.0 - self.offset.0, coord.1 - self.offset.1))
        }
    }
    fn coords(&self) -> impl Iterator<Item=(usize, usize)> {
        self.shape.coords().map(|(x, y)| (x + self.offset.0, y + self.offset.1))
    }
}

#[derive(Clone, Debug)]
struct Problem {
    height: usize,
    width: usize,
    counts: Vec<usize>
}

impl Problem {
    fn grid_size(&self) -> usize {
        self.height * self.width
    }
}


#[derive(Clone, Debug)]
struct Input {
    shapes: Vec<Shape>,
    problems: Vec<Problem>
}

fn parse(lines: &Vec<String>) -> Input {
    let mut lines = lines.iter().map(|line| line.trim()).filter(|&line| !line.is_empty());
    let mut shapes = Vec::<Shape>::new();
    let mut problems = Vec::<Problem>::new();
    let mut line = lines.next().unwrap();
    while !line.contains('x') {
        line = lines.next().unwrap();
        let mut cells = Vec::<Vec::<bool>>::new();
        while !line.contains(':') {
            cells.push(line.chars().map(|c| c == '#').collect());
            line = lines.next().unwrap();
        }
        shapes.push(Shape{cells});
    }
    loop {
        let (before, after) = line.split_once(':').unwrap();
        let (width, height) = before.split_once('x').unwrap();
        let width = width.parse::<usize>().unwrap();
        let height = height.parse::<usize>().unwrap();
        let counts: Vec<usize> = after.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        problems.push(Problem{height: height, width: width, counts: counts});
        if let Some(next_line) = lines.next() {
            line = next_line;
        } else {
            break;
        }
    }
    Input { shapes, problems }
}

fn get_tranformations(shape: &Shape) -> Vec<Shape> {
    let mut transformed: Vec<Shape> = ((0..4).map(|turns| shape.rotate(turns)).flat_map(|shape| {
        [shape.as_shape(), shape.flip().as_shape()]
    })).collect();
    transformed.sort();
    transformed.dedup();
    transformed
}

struct Grid {
    cells: Vec<Vec<bool>>
}

impl Grid {
    fn width(&self) -> usize {
        self.cells.len()
    }
    fn height(&self) -> usize {
        self.cells[0].len()
    }
    fn at(&self, coord: (usize, usize)) -> bool {
        let (x, y) = coord;
        self.cells[x][y]
    }
    fn at_mut(&mut self, coord: (usize, usize)) -> &mut bool {
        let (x, y) = coord;
        &mut self.cells[x][y]
    }
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; width];height];
        Grid{cells}
    }
}

fn get_next_coord(grid: &Grid, coord: (usize, usize)) -> Option<(usize, usize)> {
    let (mut x, mut y) = coord;
    loop {
        if x + 1 < grid.width() {
            x = x + 1;
        } else if y + 1 < grid.height() {
            x = 0;
            y = y + 1;
        } else {
            return None;
        }
        if !grid.at((x, y)) {
            return Some((x, y));
        }
    }
}

fn solve_grid(grid: &mut Grid, shapes: &Vec<(Shape, usize)>, coord: (usize, usize), remaining: &mut Vec<usize>) -> bool {
    let (x, y) = coord;
    // Try every possible combination of shape and offset to fill the coord.
    for (shape, i) in shapes {
        if remaining[*i] == 0 {
            continue;
        }
        let x_start = if x < shape.width() { 0usize } else { x + 1 - shape.width() };
        let x_end = x.min(grid.width() - shape.width()) + 1;
        let y_start = if y < shape.height() { 0usize } else { y + 1 - shape.height() };
        let y_end = y.min(grid.height() - shape.height()) + 1;
        for x_offset in x_start..x_end {
            for y_offset in y_start..y_end {
                let translated_shape = shape.translate((x_offset, y_offset));
                if translated_shape.coords().any(|coord| grid.at(coord)) {
                    // Shape cannot be placed.
                    continue;
                }
                // Place the shape and recusively fill the remaining coordintates. If
                // there are no remaining coordinates we are done.
                translated_shape.coords().for_each(|coord| (*grid.at_mut(coord)) = true);
                let Some(next_coord) = get_next_coord(grid, coord) else { return true; };
                remaining[*i] = remaining[*i] - 1;
                if solve_grid(grid, shapes, next_coord, remaining) {
                    return true;
                }
                // Otherwise undo the change and carry on searching.
                remaining[*i] = remaining[*i] + 1;
                translated_shape.coords().for_each(|coord| (*grid.at_mut(coord)) = false);
            }
        }
    }
    // If we get here the coordinate cannot be filled.
    return false
}

fn solve(shapes: &Vec<(Shape, usize)>, problem: &Problem) -> bool {
    let mut grid = Grid::new(problem.width, problem.height);
    let mut remaining = problem.counts.clone();
    solve_grid(&mut grid, shapes, (0, 0), &mut remaining)
}

fn solve_input(input: &Input) -> usize {
    // Compute all possible transformations and add an extra 1x1 shape at the end.
    let mut expanded_shapes: Vec<(Shape, usize)> = input.shapes.iter().enumerate().flat_map(|(i, shape)| {
        get_tranformations(shape).into_iter().map(move |shape| (shape, i))
    }).collect();
    expanded_shapes.push((Shape{cells: vec![vec![true]]}, input.shapes.len()));
    // Turn each problem into an exact cover problem by including the number of 1x1 shapes
    // required to completely fill the grid.
    let shape_areas: Vec<usize> = input.shapes.iter().map(|shape| shape.area()).collect();
    let problems: Vec<Problem> = input.problems.iter().flat_map(|problem| {
        let used_area: usize = problem.counts.iter().zip(shape_areas.iter()).map(|(&count, area)| count * area).sum();
        let available_area = problem.grid_size();
        if used_area > available_area {
            None
        } else {
            let unused_area = available_area - used_area;
            let mut problem = problem.clone();
            problem.counts.push(unused_area);
            Some(problem)
        }
    }).collect();
    // Solve.
    problems.iter().filter(|problem| solve(&expanded_shapes, problem)).count()
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let input: Input = parse(&lines);
    let num_solvable = solve_input(&input);
    println!("Num solvable: {}", num_solvable);
}
