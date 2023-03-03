use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
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
    let (x1, x2) = (min(x1, x2), max(x1, x2));
    let (y1, y2) = (min(y1, y2), max(y1, y2));
    Line { x1, y1, x2, y2 }
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }

    fn get_intersections(&self, other: &Line) -> Vec<(u32, u32)> {
        let h1 = self.is_horizontal();
        let h2 = other.is_horizontal();
        let v1 = self.is_vertical();
        let v2 = other.is_vertical();

        if h1 && h2 {
            if self.y1 == other.y1 {
                let start = max(self.x1, other.x1);
                let end = min(self.x2, other.x2);
                if start <= end {
                    let mut ps: Vec<(u32, u32)> = vec![];
                    for x in start..(end + 1) {
                        ps.push((x, self.y1));
                    }
                    return ps;
                }
            }
        } else if h1 && v2 {
            if (self.x1 <= other.x1)
                && (other.x1 <= self.x2)
                && (other.y1 <= self.y1)
                && (self.y1 <= other.y2)
            {
                return vec![(other.x1, self.y1)];
            }
        } else if v1 && h2 {
            if (other.x1 <= self.x1)
                && (self.x1 <= other.x2)
                && (self.y1 <= other.y1)
                && (other.y1 <= self.y2)
            {
                return vec![(self.x1, other.y1)];
            }
        } else if v1 && v2 {
            if self.x1 == other.x1 {
                let start = max(self.y1, other.y1);
                let end = min(self.y2, other.y2);
                if start < end {
                    let mut ps: Vec<(u32, u32)> = vec![];
                    for y in start..(end + 1) {
                        ps.push((self.x1, y));
                    }
                    return ps;
                }
            }
        }

        vec![]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<Line> = input.lines().map(|x| build_line(x)).collect();
    let axis_aligned_lines: Vec<&Line> = lines
        .iter()
        .filter(|x| x.is_vertical() || x.is_horizontal())
        .collect();
    let mut ps = HashMap::new();

    for l1 in &axis_aligned_lines {
        for l2 in &axis_aligned_lines {
            if l1 != l2 {
                for p in l1.get_intersections(l2) {
                    ps.insert(p, true);
                }
            }
        }
    }

    let num_intersections: u32 = ps.len().try_into().unwrap();
    Some(num_intersections)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
