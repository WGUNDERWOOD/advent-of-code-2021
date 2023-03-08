// Approach
// 1: Add one to every cell
// 2: repeat until no cells at 10:
//      for all cells at 10:
//        cell flashes, increasing neighbors which are < 10 by 1
//        cell sets to 11 (already flashed state)

use std::cmp::{min, max};

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

fn get_neighbors(i: usize, j: usize, octopuses: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = vec![];
    for ii in max(1,i)-1..=min(N-2,i)+1 {
        for jj in max(1,j)-1..=min(N-2,j)+1 {
            if (ii, jj) != (i, j) {
                neighbors.push((ii, jj))
            }
        }
    }
    neighbors
}

fn contains_10(octopuses: &Vec<Vec<u32>>) -> bool{
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

        //display(&octopuses);
        //println!("\n");

        // change all 10 to 11
        change_a_to_b(10, 11, octopuses);

        for i in 0..N {
            for j in 0..N {
                if octopuses[i][j] == 11 {
                    flashes += 1;
                    //println!("\n");
                    //println!("{:?}", (i, j));
                    //println!("");
                    for (ii, jj) in get_neighbors(i, j, octopuses) {
                        //println!("{:?}", (ii, jj));
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

fn display(octopuses: &Vec<Vec<u32>>) {
    for row in octopuses {
        println!("{:?}", row);
    }
}

pub fn part_one(input: &str) -> Option<u32> {

    let mut flashes = 0;
    let mut octopuses = parse_input(input);
    let n_steps = 100;

    for _ in 0..n_steps {
        flashes += perform_step(&mut octopuses);
        //display(&octopuses);
        //println!("\n");
    }

    Some(flashes)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
