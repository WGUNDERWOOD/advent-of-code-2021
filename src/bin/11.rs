use std::cmp::{max, min};
const N: usize = 10;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let mut octopuses: Vec<Vec<u32>> = vec![];
    for line in lines {
        let row: Vec<u32> = line
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect();
        octopuses.push(row);
    }
    octopuses
}

fn get_neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for ii in max(1, i) - 1..=min(N - 2, i) + 1 {
        for jj in max(1, j) - 1..=min(N - 2, j) + 1 {
            if (ii, jj) != (i, j) {
                neighbors.push((ii, jj))
            }
        }
    }
    neighbors
}

fn contains_10(octopuses: &Vec<Vec<u32>>) -> bool {
    let mut flag = false;
    for i in 0..N {
        for j in 0..N {
            if octopuses[i][j] == 10 {
                flag = true;
            }
        }
    }
    flag
}

fn change_a_to_b(a: u32, b: u32, octopuses: &mut Vec<Vec<u32>>) {
    for i in 0..N {
        for j in 0..N {
            if octopuses[i][j] == a {
                octopuses[i][j] = b;
            }
        }
    }
}

fn add_1(octopuses: &mut Vec<Vec<u32>>) {
    for i in 0..N {
        for j in 0..N {
            octopuses[i][j] += 1;
        }
    }
}

fn perform_step(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;
    add_1(octopuses);
    while contains_10(&octopuses) {
        change_a_to_b(10, 11, octopuses);
        for i in 0..N {
            for j in 0..N {
                if octopuses[i][j] == 11 {
                    flashes += 1;
                    for (ii, jj) in get_neighbors(i, j) {
                        if octopuses[ii][jj] < 10 {
                            octopuses[ii][jj] += 1;
                        }
                    }
                    octopuses[i][j] = 12;
                }
            }
        }
    }
    change_a_to_b(12, 0, octopuses);
    flashes
}

fn is_simultaneous_flash(octopuses: &Vec<Vec<u32>>) -> bool {
    let mut flag = true;
    for i in 0..N {
        for j in 0..N {
            if octopuses[i][j] != 0 {
                flag = false;
            }
        }
    }
    flag
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut flashes = 0;
    let mut octopuses = parse_input(input);
    let n_steps = 100;
    for _ in 0..n_steps {
        flashes += perform_step(&mut octopuses);
    }
    Some(flashes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut n_steps = 0;
    let mut simultaneous_flash = false;
    let mut octopuses = parse_input(input);
    while !simultaneous_flash {
        perform_step(&mut octopuses);
        simultaneous_flash = is_simultaneous_flash(&octopuses);
        n_steps += 1;
    }
    Some(n_steps)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(1656));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(195));
    }
}
