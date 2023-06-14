// approach
// DONE get all rotations by taking products of single-axis rotations until have 24
// translations are easy
// keep a list of "fixed" scanners starting with just the first
// for fixed scanner0 and unfixed scanner1
//     for rotation in rotations
//         rotate scanner1 by rotation
//         for beacon0 in scanner0 and beacon1 in scanner1
//             translate scanner1 so beacon1 coincides with beacon0
//             count how many beacons align
//             if this is at least 12
//                 remove scanner1 from unfixed and add it to fixed (translated rotated version)
//

use ndarray::arr2;
use ndarray::Array1;
use ndarray::Array2;

type Scanner = Array2<i32>;
type Rotation = Array2<i32>;

fn parse_scanners(input: &str) -> Vec<Scanner> {
    let scanner_strings: Vec<Vec<Vec<i32>>> = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.contains("---"))
                .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    let scanner_vecs: Vec<Vec<i32>> = scanner_strings
        .iter()
        .map(|x| x.iter().flatten().cloned().collect())
        .collect();
    let scanners: Vec<Scanner> = scanner_vecs
        .iter()
        .map(|x| Array2::from_shape_vec((x.len() / 3, 3), x.to_vec()).unwrap())
        .collect();
    return scanners;
}

fn get_rotations() -> Vec<Rotation> {
    let id = arr2(&[[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
    let rot_i = arr2(&[[1, 0, 0], [0, 0, -1], [0, 1, 0]]);
    let rot_j = arr2(&[[0, 0, -1], [0, 1, 0], [1, 0, 0]]);
    let rot_k = arr2(&[[0, -1, 0], [1, 0, 0], [0, 0, 1]]);
    let basis_rotations = vec![rot_i, rot_j, rot_k];
    let mut rotations: Vec<Rotation> = vec![id];
    for _ in 0..4 {
        for rotation in rotations.clone().iter() {
            for rot in &basis_rotations {
                let new_rotation = rotation.dot(rot);
                if !rotations.contains(&new_rotation) {
                    rotations.push(new_rotation);
                }
            }
        }
    }
    rotations
}

fn add_rows(a: Array2<i32>, b: Array1<i32>) -> Array2<i32> {
    let mut new_a = Array2::<i32>::zeros(a.raw_dim());
    for row in 0..a.nrows() {
        for col in 0..a.ncols() {
            new_a[[row, col]] += b[col];
        }
    }
    new_a
}

fn count_matches(scanner0: Scanner, scanner1: Scanner) -> i32 {
    let mut count = 0;
    for s0 in scanner0.rows() {
        let mut is_match = false;
        for s1 in scanner1.rows() {
            if s0 == s1 {
                is_match = true
            }
        }
        if is_match {
            count += 1
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    let scanners = parse_scanners(input);
    let rotations = get_rotations();
    //let mut fixed_scanners = vec![0];
    //let mut unfixed_scanners: Vec<usize> = (1..scanners.len()).collect();

    let scanner0 = &scanners[0];
    let scanner1 = &scanners[1];
    for rotation in &rotations {
        let rotated_scanner1 = scanner1.dot(rotation);
        //dbg!(&rotated_scanner1);
        for beacon0 in 0..scanner0.nrows() {
            for beacon1 in 0..scanner1.nrows() {
                //dbg!(scanner0.row(beacon0));
                let translated_scanner1 =
                    add_rows(rotated_scanner1.clone(), scanner0.row(beacon0).to_owned());
                let matches = count_matches(scanner0.clone(), translated_scanner1);
                if matches > 1 {
                    dbg!(matches);
                }
                //dbg!(beacon0);
                //dbg!(beacon1);
            }
        }
    }
    //                    dbg!(beacon0);
    //                    dbg!(beacon1);
    //                    for row in 0..scanner1.nrows() {
    //                        for col in 0..3 {
    //                            scanner1[[row, col]] +=
    //                                -scanner1[[beacon1, col]] + scanner0[[beacon0, col]];
    //                        }
    //                        dbg!(&scanner1);
    //                    }
    //                }
    //            }

    //let mut rep = 0;
    //while !unfixed_scanners.is_empty() && rep < 2 {
    //    let mut scanner1: Scanner = scanners.iter().nth(unfixed_scanners[0]).unwrap().clone();
    //    for idx0 in fixed_scanners.iter() {
    //        let scanner0: &Scanner = scanners.iter().nth(*idx0).unwrap();
    //        for rotation in &rotations {
    //            scanner1 = scanner1.dot(rotation);
    //            for beacon0 in 0..scanner0.nrows() {
    //                for beacon1 in 0..scanner1.nrows() {
    //                    dbg!(beacon0);
    //                    dbg!(beacon1);
    //                    for row in 0..scanner1.nrows() {
    //                        for col in 0..3 {
    //                            scanner1[[row, col]] +=
    //                                -scanner1[[beacon1, col]] + scanner0[[beacon0, col]];
    //                        }
    //                        dbg!(&scanner1);
    //                    }
    //                }
    //            }
    //        }
    //    }

    //    rep += 1
    //}
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(79));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
