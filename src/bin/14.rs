use std::collections::HashMap;

fn parse_input(input: &str) -> (String, HashMap<(char, char), char>) {
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let polymer = input_split[0].to_string();
    let rules_list: Vec<&str> = input_split[1].split("\n").
        filter(|x| !x.is_empty()).collect();
    let mut rules = HashMap::new();

    for rule in &rules_list {
        let pair1 = rule.chars().nth(0).unwrap();
        let pair2 = rule.chars().nth(1).unwrap();
        let c = rule.chars().nth(6).unwrap();
        rules.insert((pair1, pair2), c);
    }
    return (polymer, rules)
}

fn get_most_least_common_char(s: &str) -> (u32, u32) {
    let mut h: HashMap<char, u32> = HashMap::new();
    for c in s.chars() {
        if h.contains_key(&c) {
            *h.get_mut(&c).unwrap() += 1
        } else {
            h.insert(c, 1);
        }
    }

    let mut most_common = 0;
    let mut least_common = u32::MAX;

    for k in h.keys() {
        if h[k] > most_common {
            most_common = h[k];
        }

        if h[k] < least_common {
            least_common = h[k];
        }
    }
    (most_common, least_common)
}


fn iterate_polymer(polymer: &str, rules: &HashMap<(char, char), char>) -> String {
    let mut new_polymer = "".to_string();

    for i in 0..polymer.len()-1 {
        let c1 = polymer.chars().nth(i).unwrap();
        let c2 = polymer.chars().nth(i+1).unwrap();

        new_polymer.push(c1);

        if rules.contains_key(&(c1, c2)) {
            new_polymer.push(rules[&(c1, c2)]);
        }

    }

    let c_last = polymer.chars().last().unwrap();
    new_polymer.push(c_last);

    return new_polymer
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut polymer, rules) = parse_input(input);
    for rep in 0..10 {
        polymer = iterate_polymer(&polymer, &rules);
    }
    let (most_common, least_common) = get_most_least_common_char(&polymer);
    Some(most_common - least_common)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut polymer, rules) = parse_input(input);
    for rep in 0..20 {
        polymer = iterate_polymer(&polymer, &rules);
        println!("{}", polymer.len())
    }
    let (most_common, least_common) = get_most_least_common_char(&polymer);
    Some(most_common - least_common)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(1588));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), None);
    }
}
