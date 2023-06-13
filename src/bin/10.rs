fn get_first_illegal_char(s: &str) -> Option<char> {
    let mut delims = String::from("");
    if get_delim_type(s.chars().next().unwrap()) == Some(DelimType::Close) {
        return s.chars().next();
    }
    for c in s.chars() {
        let delim_type = get_delim_type(c).unwrap();
        match delim_type {
            DelimType::Open => delims.push_str(&String::from(c)),
            DelimType::Close => {
                if delims.is_empty() {
                    return Some(c);
                } else if delims.chars().last().unwrap() == get_match(c).unwrap() {
                    delims = delims.split_at(delims.len() - 1).0.to_string();
                } else {
                    return Some(c);
                }
            }
        }
    }
    None
}

fn get_first_illegal_char_rev(s: &str) -> Option<char> {
    let rev_match_s = s
        .chars()
        .rev()
        .map(|x| get_match(x).unwrap())
        .collect::<String>();
    get_first_illegal_char(&rev_match_s)
}

#[derive(PartialEq)]
enum DelimType {
    Open,
    Close,
}

fn get_match(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn get_delim_type(c: char) -> Option<DelimType> {
    if ['(', '[', '{', '<'].contains(&c) {
        Some(DelimType::Open)
    } else if [')', ']', '}', '>'].contains(&c) {
        Some(DelimType::Close)
    } else {
        None
    }
}

fn get_score_one(c: Option<char>) -> Option<u32> {
    match c {
        Some(')') => Some(3),
        Some(']') => Some(57),
        Some('}') => Some(1197),
        Some('>') => Some(25137),
        None => Some(0),
        _ => None,
    }
}

fn get_score_two(s: &str) -> u64 {
    let mut score = 0u64;
    for c in s.chars() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        };
    }
    score
}

fn get_completion_string(s: &str) -> String {
    let mut completed_string = s.to_string();
    let mut completion_string = "".to_string();
    let mut finished = false;
    while !finished {
        let last_illegal = get_first_illegal_char_rev(&completed_string);

        match last_illegal {
            Some(c) => {
                completed_string.push_str(&String::from(c));
                completion_string.push_str(&String::from(c));
            }
            None => finished = true,
        };
    }
    completion_string
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let first_illegals: Vec<Option<char>> =
        lines.iter().map(|x| get_first_illegal_char(x)).collect();
    let scores: Vec<u32> = first_illegals
        .iter()
        .map(|x| get_score_one(*x).unwrap())
        .collect();
    Some(scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let incompletes: Vec<&str> = lines
        .iter()
        .filter(|x| get_first_illegal_char(x).is_none())
        .copied()
        .collect();
    let mut scores: Vec<u64> = vec![];
    for incomplete in incompletes {
        let completion_string = get_completion_string(incomplete);
        let score = get_score_two(&completion_string);
        scores.push(score);
    }
    scores.sort();
    Some(scores[scores.len() / 2] as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(26397));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(288957));
    }
}
