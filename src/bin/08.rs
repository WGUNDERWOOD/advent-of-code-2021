use std::collections::HashMap;

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

fn contains_all(s1: &str, s2: &str) -> bool {
    let mut ans = true;
    for c in s2.chars() {
        if !s1.contains(c) {
            ans = false;
        }
    }
    ans
}

fn get_digits<'a>(patterns: &'a Vec<&'a str>) -> HashMap<u32, &'a str> {
    // Find 1, 4, 7, 8
    let mut digits: HashMap<u32, &str> = HashMap::new();
    for pattern in patterns {
        match pattern.len() {
            2 => digits.insert(1, pattern),
            4 => digits.insert(4, pattern),
            3 => digits.insert(7, pattern),
            7 => digits.insert(8, pattern),
            _ => None,
        };
    }
    // Find 0, 3, 6, 9
    for pattern in patterns {
        match pattern.len() {
            5 => {
                if contains_all(pattern, digits[&1]) {
                    digits.insert(3, pattern);
                };
            }
            6 => {
                if !contains_all(pattern, digits[&1]) {
                    digits.insert(6, pattern);
                } else if contains_all(pattern, digits[&4]) {
                    digits.insert(9, pattern);
                } else {
                    digits.insert(0, pattern);
                };
            }
            _ => (),
        };
    }
    // Find 2, 5
    for pattern in patterns {
        match pattern.len() {
            5 => {
                if !contains_all(pattern, digits[&1]) && contains_all(digits[&9], pattern) {
                    digits.insert(5, pattern);
                } else if !contains_all(digits[&9], pattern) {
                    digits.insert(2, pattern);
                };
            }
            _ => (),
        };
    }
    digits
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
    let (all_patterns, all_outputs) = parse_input(input);
    let mut sum_digits = 0;

    for i in 0..all_patterns.len() {
        let patterns = &all_patterns[i];
        let outputs = &all_outputs[i];
        let digits = get_digits(&patterns);
        let mut outputs_str: String = "".to_string();
        for output in outputs {
            for j in 0..=9 {
                if contains_all(&digits[&j], output) && contains_all(output, &digits[&j]) {
                    outputs_str.push_str(&j.to_string());
                }
            }
        }
        sum_digits += outputs_str.parse::<u32>().unwrap();
    }
    Some(sum_digits)
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
        assert_eq!(part_two(&input), Some(61229));
    }
}
