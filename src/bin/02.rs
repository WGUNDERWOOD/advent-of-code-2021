pub fn part_one(input: &str) -> Option<i32> {
    let ls: Vec<&str> = input.lines().collect();
    let instructions: Vec<(&str, i32)> = ls.iter().map(|l| format_input(l)).collect();
    let mut depth = 0;
    let mut horiz = 0;

    for instruction in instructions {
        (depth, horiz) = move_submarine(instruction, depth, horiz);
    }

    Some(depth * horiz)
}

pub fn part_two(input: &str) -> Option<i32> {
    let ls: Vec<&str> = input.lines().collect();
    let instructions: Vec<(&str, i32)> = ls.iter().map(|l| format_input(l)).collect();
    let mut aim = 0;
    let mut depth = 0;
    let mut horiz = 0;

    for instruction in instructions {
        (aim, depth, horiz) = aim_submarine(instruction, aim, depth, horiz);
    }

    Some(depth * horiz)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

fn format_input(l: &str) -> (&str, i32) {
    let split_l: Vec<&str> = l.split(' ').collect();
    let direction: &str = split_l[0];
    let distance: i32 = split_l[1].parse().unwrap();
    (direction, distance)
}

fn move_submarine(instruction: (&str, i32), depth: i32, horiz: i32) -> (i32, i32) {
    let mut new_depth = depth;
    let mut new_horiz = horiz;

    match instruction.0 {
        "down" => new_depth += instruction.1,
        "up" => new_depth -= instruction.1,
        "forward" => new_horiz += instruction.1,
        &_ => (),
    }

    (new_depth, new_horiz)
}

fn aim_submarine(instruction: (&str, i32), aim: i32, depth: i32, horiz: i32) -> (i32, i32, i32) {
    let mut new_aim = aim;
    let mut new_depth = depth;
    let mut new_horiz = horiz;

    match instruction.0 {
        "down" => new_aim += instruction.1,
        "up" => new_aim -= instruction.1,
        "forward" => {
            new_horiz += instruction.1;
            new_depth += aim * instruction.1;
        }
        &_ => (),
    }

    (new_aim, new_depth, new_horiz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(150));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(900));
    }
}
