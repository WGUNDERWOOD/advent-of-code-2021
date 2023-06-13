use std::cmp::{max, min};
use std::collections::HashMap;

struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

fn build_line(l: &str) -> Line {
    let mut line: Vec<&str> = l.split([',', ' ']).collect();
    line.remove(2);
    let coords: Vec<u32> = line.iter().map(|x| x.parse().unwrap()).collect();
    let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);
    Line { x1, y1, x2, y2 }
}

impl Line {
    fn is_axis_aligned(&self) -> bool {
        (self.x1 == self.x2) || (self.y1 == self.y2)
    }
}

fn add_point(ps: &mut HashMap<(u32, u32), u32>, p: (u32, u32)) {
    if let std::collections::hash_map::Entry::Vacant(e) = ps.entry(p) {
        e.insert(1);
    } else {
        *ps.get_mut(&p).unwrap() += 1;
    }
}

fn add_points(ps: &mut HashMap<(u32, u32), u32>, line: &Line) {
    let xmin = min(line.x1, line.x2);
    let xmax = max(line.x1, line.x2);
    let ymin = min(line.y1, line.y2);
    let ymax = max(line.y1, line.y2);

    if line.x1 == line.x2 {
        for y in ymin..=ymax {
            let p = (line.x1, y);
            add_point(ps, p);
        }
    } else if line.y1 == line.y2 {
        for x in xmin..=xmax {
            let p = (x, line.y1);
            add_point(ps, p);
        }
    } else {
        for x in xmin..=xmax {
            if (line.x1 < line.x2) == (line.y1 < line.y2) {
                let p = (x, x - xmin + ymin);
                add_point(ps, p);
            } else {
                let p = (x, ymax + xmin - x);
                add_point(ps, p);
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Line> = input.lines().map(build_line).collect();
    let axis_aligned_lines: Vec<&Line> = lines.iter().filter(|x| x.is_axis_aligned()).collect();
    let mut ps: HashMap<(u32, u32), u32> = HashMap::new();
    for line in axis_aligned_lines.iter() {
        add_points(&mut ps, line);
    }
    ps.retain(|_, x| *x >= 2);
    let num_intersections: u32 = ps.len().try_into().unwrap();
    Some(num_intersections)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<Line> = input.lines().map(build_line).collect();
    let mut ps: HashMap<(u32, u32), u32> = HashMap::new();
    for line in lines.iter() {
        add_points(&mut ps, line);
    }
    ps.retain(|_, x| *x >= 2);
    let num_intersections: u32 = ps.len().try_into().unwrap();
    Some(num_intersections)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(12));
    }
}
