use aoc2025::util::{get_input_path, read_lines};
use std::iter;
use std::ops;

#[derive(Copy,Clone,PartialEq)]
enum Interiority {
    Inside,
    Outside,
    InsideAbove,
    InsideBelow
}

enum Intersection {
    TouchFromAbove,
    TouchFromBelow,
    Cross,
    None
}

fn get_intersection_type(y: u32, range: &ops::Range::<u32>) -> Intersection {
    if range.contains(&y) {
        if range.start == y {
            Intersection::TouchFromAbove
        } else if range.end == y + 1 {
            Intersection::TouchFromBelow
        } else {
            Intersection::Cross
        }
    } else {
        Intersection::None
    }
}

fn intersect_range(a: &ops::Range<u32>, b: &ops::Range<u32>) -> ops::Range<u32> {
    a.start.max(b.start)..a.end.min(b.end)
}

fn inclusive_points_between(a: u32, b: u32) -> ops::Range::<u32> {
    if a <= b { a..b+1 } else { b..a+1 }
}

fn calc_area(a: [u32;2], b: [u32;2]) -> u64 {
    a.iter().copied().zip(b).map(|(a, b)| inclusive_points_between(a, b).len() as u64).product()
}

fn line_is_inside(line: (u32, ops::Range::<u32>), perpendicular_edges: &Vec<(u32, ops::Range::<u32>)>) -> bool {
    let y = line.0;
    // To determine whether the line lies inside the shape we consider the beam
    // formed by extending that line out and compute the regions along that beam
    // which are inside. The beam ends at the last perpendicular edge which is
    // guaranteed to be after the end of the line because we added extra dummy
    // perpendicular edges.
    let edges_along_beam = perpendicular_edges.iter().filter(|edge| edge.1.contains(&y));
    let regions_along_beam = edges_along_beam.scan((0u32, Interiority::Outside), |acc, edge| {
        let (start, interiority) = *acc;
        let (region, inside) = if interiority == Interiority::Outside {
            (start..edge.0, false)
        } else {
            (start..edge.0 + 1, true)
        };
        let intersection_type = get_intersection_type(y, &edge.1);
        let interiority = match (interiority, intersection_type) {
            (Interiority::Outside, Intersection::Cross) => Interiority::Inside,
            (Interiority::Outside, Intersection::TouchFromAbove) => Interiority::InsideAbove,
            (Interiority::Outside, Intersection::TouchFromBelow) => Interiority::InsideBelow,
            (Interiority::Inside, Intersection::Cross) => Interiority::Outside,
            (Interiority::Inside, Intersection::TouchFromAbove) => Interiority::InsideBelow,
            (Interiority::Inside, Intersection::TouchFromBelow) => Interiority::InsideAbove,
            (Interiority::InsideAbove, Intersection::TouchFromAbove) => Interiority::Outside,
            (Interiority::InsideAbove, Intersection::TouchFromBelow) => Interiority::Inside,
            (Interiority::InsideBelow, Intersection::TouchFromAbove) => Interiority::Inside,
            (Interiority::InsideBelow, Intersection::TouchFromBelow) => Interiority::Outside,
            _ => panic!("Unexpected state")
        };
        *acc = (region.start, interiority);
        Some((region, inside))
    });
    // Intersect the regions of the beam with the line. If any region of the
    // beam that is outside the shape intersects the line then the line is
    // outside.
    for (region, inside) in regions_along_beam {
        if !intersect_range(&region, &line.1, ).is_empty() && !inside {
            return false;
        }
        if region.end >= line.1.end {
            break
        }
    }
    true
}

fn is_inside(a: [u32;2], b: [u32;2], horizonal_edges: &Vec<(u32, ops::Range::<u32>)>, vertical_edges: &Vec<(u32, ops::Range::<u32>)>) -> bool {
    let [from_x, from_y] = a;
    let [to_x, to_y] = b;
    // Since the enclosed shape has no holes, the rectangle is contained inside
    // the shape if and only if all the lines which make up the perimeter of
    // the rectange lie inside the shape.
    line_is_inside((from_y, inclusive_points_between(from_x, to_x)) , vertical_edges) &&
    line_is_inside((to_y, inclusive_points_between(from_x, to_x)), vertical_edges) &&
    line_is_inside((from_x, inclusive_points_between(from_y, to_y)), horizonal_edges) &&
    line_is_inside((to_x, inclusive_points_between(from_y, to_y)), horizonal_edges)
}

fn main() {
    let lines = read_lines(get_input_path().join("input.txt"));
    let coords: Vec<[u32;2]> = lines.iter().map(|line| {
        let mut parts = line.split(",").map(|part| part.parse::<u32>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        [x, y]
    }).collect();
    let n = coords.len();
    let pairs = (0..n).flat_map(|j| (0..j).map(move |i| (i, j)));
    let mut areas: Vec<(u64, usize, usize)> = pairs.map(|(i, j)| {
        (calc_area(coords[i], coords[j]), i, j)
    }).collect();
    areas.sort();
    let max_area:u64 = areas.last().unwrap().0;
    println!("Max area {}", max_area);
    let final_edge: [[u32;2];2]= [*coords.last().unwrap(), *coords.first().unwrap()];
    let mut horizonal_edges = Vec::<(u32, ops::Range::<u32>)>::new();
    let mut vertical_edges = Vec::<(u32, ops::Range::<u32>)>::new();
    let edges: Vec<[[u32;2];2]> =
        coords.windows(2).map(|w| [w[0], w[1]]).chain(iter::once(final_edge).map(|w| [w[0], w[1]])).collect();
    for [[from_x, from_y],[to_x, to_y]] in edges {
        if from_x == to_x {
            vertical_edges.push((from_x, inclusive_points_between(from_y, to_y)));
        } else {
            horizonal_edges.push((from_y, inclusive_points_between(from_x, to_x)));
        }
    }
    let max_x = coords.iter().map(|coord| coord[0]).max().unwrap();
    let max_y = coords.iter().map(|coord| coord[1]).max().unwrap();
    // Add extra edges after all the points horizontally and vertically. This
    // ensures any region we need to query is always bounded from above by
    // perpendicular edge, simplifying the line_is_inside() function.
    horizonal_edges.push((max_y + 1, (0..max_x + 1)));
    vertical_edges.push((max_x + 1, (0..max_y + 1)));
    horizonal_edges.sort_by_key(|x| (x.0, x.1.start));
    vertical_edges.sort_by_key(|x| (x.0, x.1.start));
    let mut max_inclosed_area = None;
    for &(area, a, b) in areas.iter().rev() {
        if is_inside(coords[a], coords[b], &horizonal_edges, &vertical_edges) {
            max_inclosed_area = Some(area);
            break;
        }
    }
    println!("Max inclosed area {}", max_inclosed_area.unwrap());
}
