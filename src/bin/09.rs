use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let mut height_map: Vec<Vec<u32>> = vec![];
    for line in lines {
        let row: Vec<u32> = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        height_map.push(row);
    }
    height_map
}

fn get_neighbors(i: usize, j: usize, height_map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    let ni = height_map.len();
    let nj = height_map[0].len();
    if i > 0 {
        neighbors.push((i - 1, j))
    }
    if i < ni - 1 {
        neighbors.push((i + 1, j))
    }
    if j > 0 {
        neighbors.push((i, j - 1))
    }
    if j < nj - 1 {
        neighbors.push((i, j + 1))
    }
    neighbors
}

fn is_low_point(i: usize, j: usize, height_map: &Vec<Vec<u32>>) -> bool {
    let mut low_point = true;
    let neighbors = get_neighbors(i, j, height_map);
    for neighbor in neighbors {
        if height_map[neighbor.0][neighbor.1] <= height_map[i][j] {
            low_point = false;
        }
    }
    low_point
}

fn get_destination(i: usize, j: usize, height_map: &Vec<Vec<u32>>) -> (usize, usize) {
    let mut new_i = i;
    let mut new_j = j;

    for _ in 0..10 {
        let neighbors = get_neighbors(new_i, new_j, &height_map);
        for neighbor in neighbors {
            if height_map[neighbor.0][neighbor.1] < height_map[new_i][new_j] {
                new_i = neighbor.0;
                new_j = neighbor.1;
            }
        }
    }
    (new_i, new_j)
}

pub fn part_one(input: &str) -> Option<u32> {
    let height_map = parse_input(input);
    let mut risk_level: u32 = 0;
    let ni = height_map.len();
    let nj = height_map[0].len();
    for i in 0..ni {
        for j in 0..nj {
            if is_low_point(i, j, &height_map) {
                risk_level += 1 + height_map[i][j]
            }
        }
    }
    Some(risk_level)
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map = parse_input(input);
    let ni = height_map.len();
    let nj = height_map[0].len();
    let mut basins: HashMap<(usize, usize), u32> = HashMap::new();
    for i in 0..ni {
        for j in 0..nj {
            if height_map[i][j] != 9 {
                let (dest_i, dest_j) = get_destination(i, j, &height_map);

                if basins.contains_key(&(dest_i, dest_j)) {
                    *basins.get_mut(&(dest_i, dest_j)).unwrap() += 1;
                } else {
                    basins.insert((dest_i, dest_j), 1);
                }
            }
        }
    }

    let mut basin_sizes: Vec<u32> = basins.values().cloned().collect();
    basin_sizes.sort();
    basin_sizes.reverse();
    let product_three_largest_basin_sizes = basin_sizes[0..3].iter().product();
    Some(product_three_largest_basin_sizes)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1134));
    }
}
