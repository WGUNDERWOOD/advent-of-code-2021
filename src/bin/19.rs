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
//
// new approach
//
// for each scanner construct a beacon to beacon difference array
// with distances[(beacon0, beacon1)] = beacon1 - beacon0
//
// for each scanner construct a beacon to beacon distance array
// with distances[(beacon0, beacon1)] = ||beacon1 - beacon0||_1
//
// for each pair of scanners check the overlap of their distance arrays
// and record these in overlaps[scanner0, scanner1]
//
// determine the pairings by finding those with at least 12 overlaps
//
// for each pairing use the difference arrays to find the rotation and translation
// giving at least 12 refined overlaps, traversing the path of pairings
//
//
// another new approach
//
// for each scanner construct a fingerprint
// this is a list of all squared distances between beacons detected
//
// then for each pair of scanner see if at least 66 of these agree,
// store such pairs in a list
//
// TODO then work out rotations and translations
// TODO then count total number of beacons

type Point = (i32, i32, i32);
type Scanner = Vec<Point>;
type Fingerprint = Vec<i32>;

fn parse_scanners(input: &str) -> Vec<Scanner> {
    let scanners_list: Vec<Vec<Vec<i32>>> = input
        .split("\n\n")
        .map(|x| {
            x.split("\n")
                .filter(|x| !x.contains("---"))
                .map(|x| x.split(",").map(|x| x.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    let scanners = scanners_list
        .iter()
        .map(|x| x.iter().map(|y| (y[0], y[1], y[2])).collect())
        .collect();
    scanners
}

fn get_distance(p0: Point, p1: Point) -> i32 {
    return (p0.0 - p1.0).pow(2) + (p0.1 - p1.1).pow(2) + (p0.2 - p1.2).pow(2);
}

fn get_fingerprints(scanners: Vec<Scanner>) -> Vec<Fingerprint> {
    let mut fingerprints = vec![];
    for s in scanners {
        let n = s.len();
        let mut distances = vec![];
        for i in 0..n {
            for j in 0..n {
                if i < j {
                    let bi = s[i];
                    let bj = s[j];
                    distances.push(get_distance(bi, bj));
                }
            }
        }
        fingerprints.push(distances);
    }
    fingerprints
}

/*
fn get_distances(scanners: &Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
let mut distances: Vec<Vec<Vec<i32>>> = vec![];
for scanner in 0..scanners.len() {
let n_beacons = scanners[scanner].len();
distances.push(vec![vec![0; n_beacons]; n_beacons]);
for beacon0 in 0..n_beacons {
for beacon1 in 0..n_beacons {
for i in 0..3 {
let distance = (scanners[scanner][beacon1][i] - scanners[scanner][beacon1][i]).abs();
distances[scanner][beacon0][beacon1] += distance;
}
}
}
}
distances
}
*/

pub fn part_one(input: &str) -> Option<u32> {
    let scanners = parse_scanners(input);
    dbg!(&scanners);
    let fingerprints = get_fingerprints(scanners);
    dbg!(&fingerprints);
    //let distances = get_distances(&scanners);
    //dbg!(&distances[0]);
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
