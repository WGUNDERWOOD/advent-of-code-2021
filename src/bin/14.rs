use std::collections::HashMap;
type Polymer = HashMap<(char, char), u64>;
type PolymerCounts = HashMap<char, u64>;
type Rules = HashMap<(char, char), char>;

fn insert_or_increment(polymer: &mut Polymer, c: (char, char), increment: u64) {
    if polymer.contains_key(&c) {
        *polymer.get_mut(&c).unwrap() += increment;
    } else {
        polymer.insert(c, increment);
    }
}

fn insert_or_increment_counts(polymer_counts: &mut PolymerCounts, c: char, increment: u64) {
    if polymer_counts.contains_key(&c) {
        *polymer_counts.get_mut(&c).unwrap() += increment;
    } else {
        polymer_counts.insert(c, increment);
    }
}

fn parse_polymer_counts(input: &str) -> PolymerCounts {
    let polymer_list: Vec<char> = input.split("\n\n").collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    let mut polymer_counts: PolymerCounts = HashMap::new();
    for i in 0..polymer_list.len() {
        let c = polymer_list[i];
        insert_or_increment_counts(&mut polymer_counts, c, 1)
    }
    polymer_counts
}

fn parse_polymer(input: &str) -> Polymer {
    let polymer_list: Vec<char> = input.split("\n\n").collect::<Vec<&str>>()[0]
        .chars()
        .collect();
    let mut polymer: Polymer = HashMap::new();
    for i in 0..polymer_list.len() - 1 {
        let c1 = polymer_list[i];
        let c2 = polymer_list[i + 1];
        insert_or_increment(&mut polymer, (c1, c2), 1)
    }
    polymer
}

fn parse_rules(input: &str) -> Rules {
    let rules_list: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>()[1]
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();
    let mut rules = HashMap::new();
    for rule in &rules_list {
        let pair1 = rule.chars().nth(0).unwrap();
        let pair2 = rule.chars().nth(1).unwrap();
        let c = rule.chars().nth(6).unwrap();
        rules.insert((pair1, pair2), c);
    }
    return rules;
}

fn get_most_least_common_char(polymer_counts: &PolymerCounts) -> (u64, u64) {
    let mut most_common = 0;
    let mut least_common = u64::MAX;
    for (_, n) in polymer_counts {
        if *n > most_common {
            most_common = *n;
        }
        if *n < least_common {
            least_common = *n;
        }
    }
    (most_common, least_common)
}

fn iterate_polymer(polymer: &mut Polymer, polymer_counts: &mut PolymerCounts, rules: &Rules) {
    for ((c1, c2), n) in polymer.clone() {
        if rules.contains_key(&(c1, c2)) {
            let c3 = rules[&(c1, c2)];
            if rules.contains_key(&(c1, c3)) {
                insert_or_increment(polymer, (c1, c3), n);
            }
            if rules.contains_key(&(c3, c2)) {
                insert_or_increment(polymer, (c3, c2), n);
            }
            *polymer.get_mut(&(c1, c2)).unwrap() -= n;
            insert_or_increment_counts(polymer_counts, c3, n);
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut polymer = parse_polymer(input);
    let mut polymer_counts = parse_polymer_counts(input);
    let rules = parse_rules(input);
    for _rep in 0..10 {
        iterate_polymer(&mut polymer, &mut polymer_counts, &rules);
    }
    let (most_common, least_common) = get_most_least_common_char(&polymer_counts);
    Some(most_common - least_common)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut polymer = parse_polymer(input);
    let mut polymer_counts = parse_polymer_counts(input);
    let rules = parse_rules(input);
    for _rep in 0..40 {
        iterate_polymer(&mut polymer, &mut polymer_counts, &rules);
    }
    let (most_common, least_common) = get_most_least_common_char(&polymer_counts);
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
        assert_eq!(part_two(&input), Some(2188189693529));
    }
}
