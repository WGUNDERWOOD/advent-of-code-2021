use std::cmp::min;

fn get_median(v: &Vec<u32>) -> u32 {
    let mut sv = v.clone();
    sv.sort();
    sv[v.len() / 2]
}

fn fuel_part_one(a: u32, positions: &Vec<u32>) -> u32 {
    let mut fuel: u32 = 0;
    for position in positions {
        fuel += (*position as i32 - a as i32).abs() as u32
    }
    fuel
}

fn fuel_part_two(a: u32, positions: &Vec<u32>) -> u32 {
    let mut fuel: u32 = 0;
    for position in positions {
        let deviation = (*position as i32 - a as i32).abs() as u32;
        fuel += deviation * (deviation + 1) / 2
    }
    fuel
}

pub fn part_one(input: &str) -> Option<u32> {
    let positions: Vec<u32> = input
        .split([',', '\n'])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let median = get_median(&positions);
    let fuel = fuel_part_one(median, &positions);
    Some(fuel)
}

pub fn part_two(input: &str) -> Option<u32> {
    let positions: Vec<u32> = input
        .split([',', '\n'])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let median = get_median(&positions);
    let mut best_fuel = fuel_part_two(median, &positions);

    for a in 0..=positions.len() {
        let fuel = fuel_part_two(a as u32, &positions);
        best_fuel = min(fuel, best_fuel);
    }
    Some(best_fuel)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(168));
    }
}
