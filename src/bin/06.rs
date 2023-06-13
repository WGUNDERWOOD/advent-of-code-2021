use ndarray::{arr1, arr2, Array1, Array2};

const NSTATES: usize = 9;

fn parse_input(input: &str) -> Array1<u64> {
    let input_ints: Vec<u64> = input
        .split([',', '\n'])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    let mut v0 = [0u64; NSTATES];
    for i in input_ints.iter() {
        v0[*i as usize] += 1;
    }
    arr1(&v0)
}

fn build_a() -> Array2<u64> {
    let mut a = [[0; NSTATES]; NSTATES];
    for (i, b) in a.iter_mut().enumerate().take(NSTATES - 1) {
        b[i + 1] = 1;
    }
    a[6][0] = 1;
    a[8][0] = 1;
    arr2(&a)
}

fn matrix_power(a: &Array2<u64>, n: u64) -> Array2<u64> {
    let mut b = a.clone();
    for _ in 1..n {
        b = a.dot(&b)
    }
    b
}

pub fn part_one(input: &str) -> Option<u64> {
    let n = 80;
    let v0 = parse_input(input);
    let a = build_a();
    let an = matrix_power(&a, n);
    let vn = an.dot(&v0);
    let total = vn.sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let n = 256;
    let v0 = parse_input(input);
    let a = build_a();
    let an = matrix_power(&a, n);
    let vn = an.dot(&v0);
    let total = vn.sum();
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5934));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(26984457539));
    }
}
