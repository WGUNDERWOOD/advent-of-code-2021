fn is_int(s: &str) -> bool {
    let flags: Vec<bool> = s.chars().map(|c| c.is_ascii_digit()).collect();
    !flags.contains(&false)
}

fn explode(snailfish: &mut Vec<String>) -> bool {
    let mut bracket_count = 0;
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
        if bracket_count >= 5 && is_int(&snailfish[i + 1]) {
            explode_comma_location = i + 2;
            flag = true;
            break;
        };
    }

    if !flag {
        return false;
    }

    // find previous int
    let mut previous_int_location = 0;
    for (i, s) in snailfish
        .iter()
        .enumerate()
        .take(explode_comma_location - 1)
    {
        if is_int(s) {
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
    for (i, s) in snailfish
        .iter()
        .enumerate()
        .skip(explode_comma_location + 2)
    {
        if is_int(s) {
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
    for i in 0..4 {
        snailfish[explode_comma_location - 2 + i] = "".to_string();
    }
    snailfish[explode_comma_location + 2] = "0".to_string();
    snailfish.retain(|x| !x.is_empty());
    flag
}

fn split(snailfish: &mut Vec<String>) -> bool {
    let mut flag = false;
    let mut int_value: i32 = 0;

    // find where to split
    let mut split_location = 0;
    for (i, s) in snailfish.iter().enumerate() {
        if is_int(s) {
            int_value = s.parse().unwrap();
            if int_value >= 10 {
                split_location = i;
                flag = true;
                break;
            }
        };
    }

    if !flag {
        return false;
    };

    let new_left = int_value / 2;
    let new_right = (int_value + 1) / 2;
    snailfish[split_location] = "]".to_string();
    snailfish.insert(split_location, new_right.to_string());
    snailfish.insert(split_location, ",".to_string());
    snailfish.insert(split_location, new_left.to_string());
    snailfish.insert(split_location, "[".to_string());
    flag
}

fn reduce(snailfish: &mut Vec<String>) -> bool {
    if explode(snailfish) {
        return true;
    };
    if split(snailfish) {
        return true;
    };
    false
}

fn get_magnitude(snailfish: &mut Vec<String>) -> i32 {
    let value: i32;

    // if just one value, return it
    if snailfish.len() == 1 {
        value = snailfish[0].parse().unwrap();
        return value;
    };

    // otherwise find central comma
    let mut central_comma_location = 0;
    let mut bracket_count = 0;
    for (i, s) in snailfish.iter().enumerate().skip(1) {
        if s == "[" {
            bracket_count += 1
        };
        if s == "]" {
            bracket_count -= 1
        };
        if bracket_count == 0 {
            central_comma_location = i + 1;
            break;
        };
    }

    // recurse
    let left = &mut snailfish[1..central_comma_location].to_vec();
    let right = &mut snailfish[central_comma_location + 1..snailfish.len() - 1].to_vec();
    value = 3 * get_magnitude(left) + 2 * get_magnitude(right);
    value
}

pub fn part_one(input: &str) -> Option<u32> {
    let snailfishes: Vec<Vec<String>> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    let mut snailfish = snailfishes[0].clone();
    for s in snailfishes.iter().skip(1) {
        let mut summand = s.clone();
        snailfish.insert(0, "[".to_string());
        snailfish.push(",".to_string());
        snailfish.append(&mut summand);
        snailfish.push("]".to_string());
        while reduce(&mut snailfish) {}
    }
    let magnitude = get_magnitude(&mut snailfish);
    Some(magnitude.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let snailfishes: Vec<Vec<String>> = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split("")
                .filter(|x| !x.is_empty())
                .map(|x| x.to_string())
                .collect()
        })
        .collect();
    let mut best_magnitude = 0;
    for i in 0..snailfishes.len() {
        for j in 0..snailfishes.len() {
            if i != j {
                let mut snailfish = snailfishes[i].clone();
                let mut summand = snailfishes[j].clone();
                snailfish.insert(0, "[".to_string());
                snailfish.push(",".to_string());
                snailfish.append(&mut summand);
                snailfish.push("]".to_string());
                while reduce(&mut snailfish) {}
                let magnitude = get_magnitude(&mut snailfish);
                if magnitude > best_magnitude {
                    best_magnitude = magnitude
                }
            }
        }
    }
    Some(best_magnitude.try_into().unwrap())
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
        assert_eq!(part_two(&input), Some(3993));
    }
}
