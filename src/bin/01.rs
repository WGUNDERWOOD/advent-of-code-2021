pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<i32> = input.lines().map(|n| n.parse().unwrap()).collect();
    let mut counter = 0;
    for i in 1..lines.len() {
        if lines[i] > lines[i - 1] {
            counter += 1
        }
    }
    return Some(counter);
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
