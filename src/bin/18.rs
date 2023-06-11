struct Snailfish {
    id: String,
    left: Option<i32>,
    right: Option<i32>,
    value: Option<i32>,
}

fn get_middle(s: &str) -> usize {
    let mut bracket_count = 0;
    let mut middle = 0;
    for i in 1..s.len()-1 {
        let c = s.chars().nth(i);
        if c == Some('[') {bracket_count += 1};
        if c == Some(']') {bracket_count -= 1};
        if bracket_count == 0 {
            middle = i + 1;
            break;
        };
    };
    return middle
}

fn parse_snailfish(s: &str) -> Snailfish {
    // snailfish with no children
    if s.len() == 3 {
        let id = s.to_string();
        let left = None;
        let right = None;
        let value = Some(s.chars().nth(1).unwrap().to_digit(10) as i32);
        return Snailfish{id, left, right, value}
    } else {
        let middle = get_middle(s);
        let mut left: Option<i32> = None;
        let mut right: Option<i32> = None;
        let lchar = s.chars().nth(1).unwrap();
        let rchar = s.chars().nth(middle + 1).unwrap();
        if lchar.is_digit(10) {
            left = lchar;
        } else {
            left = parse_snailfish(&s[1..middle])
        }

    dbg!(middle);

    }


    let snailfish = Snailfish{id: "hi", left: None, right: None, value: None};
    return snailfish
}

fn parse_snailfishes(input: &str) -> Vec<Snailfish> {
    return input.split("\n").map(|s| parse_snailfish(s)).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let snailfishes = parse_snailfishes(input);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(4140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
