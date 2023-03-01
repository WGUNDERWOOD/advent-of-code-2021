fn get_most_common_bit(binaries: &Vec<Vec<bool>>) -> Vec<bool> {

    let mut most_common_bit: Vec<bool> = vec![];
    let n = binaries.len();
    let l = binaries[0].len();

    for j in 0..l {
        let mut counter = 0;
        for i in 0..n {
            if binaries[i][j] {
                counter += 1;
            }
        }
        most_common_bit.push(counter > n - counter);
    }

    most_common_bit
}

fn to_decimal(binary: &Vec<bool>) -> u32 {
    let mut decimal: u32 = 0;
    let l = binary.len();
    for j in 0..l {
        let power: u32 = (l - j - 1).try_into().unwrap();
        if binary[j] {
            decimal += 2_u32.pow(power)
        }
    }
    decimal
}

pub fn part_one(input: &str) -> Option<u32> {
    let ls: Vec<&str> = input.lines().collect();
    let mut binaries: Vec<Vec<bool>> = vec![];

    for l in ls.iter() {
        let mut binary: Vec<bool> = vec![];
        for c in l.chars() {
            binary.push(c == '1')
        }
        binaries.push(binary)
    }

    let most_common_bit = get_most_common_bit(&binaries);
    let gamma = to_decimal(&most_common_bit);

    let least_common_bit: Vec<bool> = most_common_bit.clone().iter().map(|x| !x).collect();
    let epsilon = to_decimal(&least_common_bit);

    Some(gamma * epsilon)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(198));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
