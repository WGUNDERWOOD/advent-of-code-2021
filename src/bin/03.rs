fn get_most_common(binaries: &Vec<Vec<bool>>) -> Vec<bool> {
    let mut most_common: Vec<bool> = vec![];
    let n = binaries.len();
    let l = binaries[0].len();

    for j in 0..l {
        let mut counter = 0;
        for binary in binaries.iter() {
            if binary[j] {
                counter += 1;
            }
        }
        most_common.push(counter >= n - counter);
    }
    most_common
}

fn to_decimal(binary: &Vec<bool>) -> u32 {
    let mut decimal: u32 = 0;
    let l = binary.len();
    for (j, b) in binary.iter().enumerate() {
        let power: u32 = (l - j - 1).try_into().unwrap();
        if *b {
            decimal += 2_u32.pow(power)
        }
    }
    decimal
}

fn get_binaries(input: &str) -> Vec<Vec<bool>> {
    let ls: Vec<&str> = input.lines().collect();
    let mut binaries: Vec<Vec<bool>> = vec![];

    for l in ls.iter() {
        let mut binary: Vec<bool> = vec![];
        for c in l.chars() {
            binary.push(c == '1')
        }
        binaries.push(binary)
    }
    binaries
}

enum MostLeast {
    Most,
    Least,
}

fn filter_most_common(binaries: &Vec<Vec<bool>>, mostleast: MostLeast) -> Vec<bool> {
    let n: usize = binaries.len();
    let mut ind_list: Vec<usize> = (0..n).collect();
    let mut j = 0;

    while ind_list.len() > 1 {
        let k: usize = ind_list.len();
        let mut counter = 0;

        for i in 0..k {
            if binaries[ind_list[i]][j] {
                counter += 1;
            }
        }

        let common = match mostleast {
            MostLeast::Most => counter >= k.wrapping_sub(counter),
            MostLeast::Least => counter < k.wrapping_sub(counter),
        };

        ind_list.retain(|x| binaries[*x][j] == common);
        j += 1;
    }

    let ind = ind_list[0];
    binaries[ind].clone()
}

pub fn part_one(input: &str) -> Option<u32> {
    let binaries = get_binaries(input);
    let most_common = get_most_common(&binaries);
    let gamma = to_decimal(&most_common);
    let least_common: Vec<bool> = most_common.iter().map(|x| !x).collect();
    let epsilon = to_decimal(&least_common);
    Some(gamma * epsilon)
}

pub fn part_two(input: &str) -> Option<u32> {
    let binaries = get_binaries(input);
    let filtered_most_common = filter_most_common(&binaries, MostLeast::Most);
    let oxygen = to_decimal(&filtered_most_common);
    let filtered_least_common = filter_most_common(&binaries, MostLeast::Least);
    let co2 = to_decimal(&filtered_least_common);
    Some(oxygen * co2)
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
        assert_eq!(part_two(&input), Some(230));
    }
}
