fn get_first_illegal_char(s: &str) -> Option<char> {

    let mut delims = String::from("");

    if get_delim_type(s.chars().next().unwrap()) == Some(DelimType::Close) {
        return s.chars().next()
    }

    for c in s.chars() {
        let delim_type = get_delim_type(c).unwrap();

        match delim_type {
            DelimType::Open => delims.push_str(&String::from(c)),
            DelimType::Close => {
                if delims.chars().last().unwrap() == get_match(c).unwrap() {
                    delims = delims.split_at(delims.len() - 1).0.to_string();
                } else {
                    return Some(c);
                }
            }
        }
    }
    None
}

#[derive(PartialEq)]
enum DelimType {
    Open,
    Close,
}

fn get_match(c: char) -> Option<char> {
    match c {
        '(' => return Some(')'),
        '[' => return Some(']'),
        '{' => return Some('}'),
        '<' => return Some('>'),
        ')' => return Some('('),
        ']' => return Some('['),
        '}' => return Some('{'),
        '>' => return Some('<'),
        _ => return None,
    }
}

fn get_delim_type(c: char) -> Option<DelimType> {
    if ['(', '[', '{', '<'].contains(&c) {
        return Some(DelimType::Open);
    } else if [')', ']', '}', '>'].contains(&c) {
        return Some(DelimType::Close);
    } else {
        return None;
    }
}

fn get_error_score(c: Option<char>) -> Option<u32> {
    match c {
        Some(')') => Some(3),
        Some(']') => Some(57),
        Some('}') => Some(1197),
        Some('>') => Some(25137),
        None => Some(0),
        _ => None,
    }
}

fn get_completion_string(s: &str) -> &str {

    let mut j = 0;

    while get_first_illegal_char(&s.chars().rev().collect::<String>()).is_some() && j < 100 {
        let last_illegal = get_first_illegal_char(&s.chars().rev().collect::<String>());
        j += 1;
    }

    println!("{}", &s);
    println!("{}", &s.chars().rev().collect::<String>());

    //let mut incomplete_string = s.clone().to_string();
    //let mut completion_string = "".to_string();

    //while incomplete_string.len() > 0 {
        //let next_char = get_match(incomplete_string.chars().last().unwrap()).unwrap();
        //println!("{:?}", incomplete_string);
        //println!("{}", next_char);
        //completion_string.push(next_char);
        //let new_incomplete_string = incomplete_string
            //.split_at(incomplete_string.len() - 1)
            //.0
            //.to_string();
        //incomplete_string = new_incomplete_string;
    //}

    //completion_string.to_string()
    ""
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let first_illegals: Vec<Option<char>> =
        lines.iter().map(|x| get_first_illegal_char(x)).collect();
    let error_scores: Vec<u32> = first_illegals
        .iter()
        .map(|x| get_error_score(*x).unwrap())
        .collect();
    Some(error_scores.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.split('\n').filter(|x| !x.is_empty()).collect();
    let incompletes: Vec<&str> = lines
        .iter()
        .filter(|x| get_first_illegal_char(x).is_none())
        .copied()
        .collect();
    let incomplete = incompletes[0];
    let completion_string = get_completion_string(incomplete);
    println!("{:?}", completion_string);
    //println!("{:?}", incompletes);

    None
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
