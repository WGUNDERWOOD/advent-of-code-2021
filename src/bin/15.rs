use core::f32::INFINITY;
use std::collections::HashMap;

fn parse_cavern(input: &str) -> Vec<Vec<i32>> {
    let cavern_rows = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let n = cavern_rows.len();
    let mut cavern = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            cavern[i][j] = cavern_rows[i]
                .chars()
                .nth(j)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .unwrap()
        }
    }
    return cavern;
}

fn get_enlarged_cavern(cavern: &Vec<Vec<i32>>, factor: i32) -> Vec<Vec<i32>> {

    let n = cavern.len();
    let enlarged_n = factor as usize * n;
    let mut enlarged_cavern: Vec<Vec<i32>> = vec![vec![0; enlarged_n]; enlarged_n];

    for i in 0..enlarged_n {
        for j in 0..enlarged_n {
            let increment = (i / n + j / n) as i32;
            enlarged_cavern[i][j] = (cavern[i % n][j % n] + increment - 1) % 9 + 1;
        }
    }
    return enlarged_cavern
}

fn get_neighbors(current: (i32, i32), n: i32) -> HashMap<(i32, i32), bool> {
    let mut neighbors: HashMap<(i32, i32), bool> = HashMap::new();
    for i in [current.0 - 1, current.0 + 1] {
        if (0 <= i) && (i <= n - 1) {
            neighbors.insert((i, current.1), true);
        }
    }
    for j in [current.1 - 1, current.1 + 1] {
        if (0 <= j) && (j <= n - 1) {
            neighbors.insert((current.0, j), true);
        }
    }
    return neighbors;
}

fn get_best_risk(cavern: &Vec<Vec<i32>>) -> i32 {
    let n = cavern.len();

    // initialize unvisited set
    let mut unvisited: HashMap<(i32, i32), bool> = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            unvisited.insert((i as i32, j as i32), true);
        }
    }

    // initialize distances and current position
    let mut distances: Vec<Vec<f32>> = vec![vec![INFINITY; n]; n];
    distances[0][0] = 0.0;
    let mut current: (i32, i32) = (0, 0);
    let mut terminated = false;

    // dijkstra's algorithm
    while !terminated {
        for neighbor in get_neighbors(current, n as i32).keys() {
            if unvisited.contains_key(neighbor) {
                let tentative_distance = distances[current.0 as usize][current.1 as usize]
                    + cavern[neighbor.0 as usize][neighbor.1 as usize] as f32;
                if tentative_distance <= distances[neighbor.0 as usize][neighbor.1 as usize] {
                    distances[neighbor.0 as usize][neighbor.1 as usize] = tentative_distance
                }
            }
        }

        unvisited.remove(&current);

        if unvisited.contains_key(&(n as i32 - 1, n as i32 - 1)) {
            let mut closest_unvisited: f32 = INFINITY;
            for point in unvisited.keys() {
                let new_dist = distances[point.0 as usize][point.1 as usize];
                if new_dist < closest_unvisited {
                    current = *point;
                    closest_unvisited = new_dist;
                }
            }
        } else {
            terminated = true;
        }
    }

    return distances[n - 1][n - 1] as i32;
}

pub fn part_one(input: &str) -> Option<u32> {
    let cavern = parse_cavern(input);
    let best_risk = get_best_risk(&cavern);
    return Some(best_risk as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let cavern = parse_cavern(input);
    let enlarged_cavern = get_enlarged_cavern(&cavern, 2);
    let best_risk = get_best_risk(&enlarged_cavern);
    return Some(best_risk as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(40));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(315));
    }
}
