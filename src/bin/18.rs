fn is_int(s: &String) -> bool {
    let flags: Vec<bool> = s.chars().map(|c| c.is_digit(10)).collect();
    return !flags.contains(&false);
}

fn explode(snailfish: &mut Vec<String>) -> bool {
    let mut bracket_count = 0;
    let mut next_int_location = 0;
    let mut flag = false;

    // find where to explode
    let mut explode_comma_location = 0;
    for i in 0..snailfish.len() {
        let c = &snailfish[i];
        if c == "[" {
            bracket_count += 1
        };
        if c == "]" {
            bracket_count -= 1
        };
        if bracket_count >= 5 && is_int(&snailfish[i+1]) {
            explode_comma_location = i + 2;
            flag = true;
            break;
        };
    }

    // find previous int
    let mut previous_int_location = 0;
    for i in 0..explode_comma_location - 1 {
        if is_int(&snailfish[i]) {
            previous_int_location = i;
        };
    }

    // add to previous int
    if previous_int_location > 0 {
        let previous_int: i32 = snailfish[previous_int_location].parse().unwrap();
        let explode_left_int: i32 = snailfish[explode_comma_location - 1].parse().unwrap();
        let new_previous_int = previous_int + explode_left_int;
        snailfish[previous_int_location] = new_previous_int.to_string();
    }

    // find next int
    let mut next_int_location = 0;
    for i in explode_comma_location + 2..snailfish.len() {
        if is_int(&snailfish[i]) {
            next_int_location = i;
            break;
        };
    }

    // add to next int
    if next_int_location > 0 {
        let next_int: i32 = snailfish[next_int_location].parse().unwrap();
        let explode_right_int: i32 = snailfish[explode_comma_location + 1].parse().unwrap();
        let new_next_int = next_int + explode_right_int;
        snailfish[next_int_location] = new_next_int.to_string();
    }

    // drop old pair
    snailfish[explode_comma_location - 1] = "0".to_string();
    snailfish[explode_comma_location] = "".to_string();
    snailfish[explode_comma_location + 1] = "".to_string();
    snailfish.retain(|x| !x.is_empty());
    return flag;
}

pub fn part_one(input: &str) -> Option<u32> {
    let snailfishes: Vec<Vec<String>> = input
        .split("\n")
        .map(|x| x.split("").filter(|x| !x.is_empty()).map(|x| x.to_string()).collect())
        .collect();
    let mut snailfish = snailfishes[0].clone();
    //let mut snailfish = "".to_string();
    //for summand in snailfishes.iter() {
    //snailfish.push_str(summand);
    //explode(&mut snailfish);
    //}
    dbg!(&snailfish);
    explode(&mut snailfish);
    dbg!(snailfish);
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
