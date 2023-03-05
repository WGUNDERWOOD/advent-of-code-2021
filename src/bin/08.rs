fn parse_input(input: &str) -> (Vec<Vec<&str>>, Vec<Vec<&str>>) {
    let lines = input.split('\n').filter(|x| !x.is_empty());
    let mut all_patterns: Vec<Vec<&str>> = vec![];
    let mut all_outputs: Vec<Vec<&str>> = vec![];
    for line in lines {
        let line_split: Vec<&str> = line.split('|').collect();
        let patterns: Vec<&str> = line_split[0].split(' ').filter(|x| !x.is_empty()).collect();
        let outputs: Vec<&str> = line_split[1].split(' ').filter(|x| !x.is_empty()).collect();
        all_patterns.push(patterns);
        all_outputs.push(outputs);
    }
    (all_patterns, all_outputs)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut n_unique = 0;
    let (_, all_outputs) = parse_input(input);
    for outputs in all_outputs {
        for output in outputs {
            if [2, 3, 4, 7].contains(&output.len()) {
                n_unique += 1;
            }
        }
    }
    Some(n_unique)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
