type Scanner = Vec<Vec<i32>>;

fn parse_scanners(input: &str) -> Vec<Scanner> {
    let scanners: Vec<Scanner> = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.contains("---"))
                .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    return scanners;
}

pub fn part_one(input: &str) -> Option<u32> {
    let scanners = parse_scanners(input);
    dbg!(&scanners[0]);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(79));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
