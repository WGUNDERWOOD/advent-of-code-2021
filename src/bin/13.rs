use std::collections::HashMap;

fn get_dot(s: &str) -> Dot {
    let dot_vec: Vec<u32> = s.split(',').map(|x| x.parse().unwrap()).collect();
    Dot {
        x: dot_vec[0],
        y: dot_vec[1],
    }
}

fn get_fold(s: &str) -> Fold {
    let s_split: Vec<&str> = s.split([' ', '=']).collect();
    let direction: Direction = match s_split[2] {
        "x" => Direction::X,
        "y" => Direction::Y,
        _ => panic!(),
    };
    let position: u32 = s_split[3].parse().unwrap();
    Fold {
        direction,
        position,
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Dot {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
struct Fold {
    direction: Direction,
    position: u32,
}

#[derive(Debug, Clone)]
enum Direction {
    X,
    Y,
}

fn display(dots: &HashMap<Dot, bool>) {
    let x_max: u32 = dots.keys().map(|k| k.x).max().unwrap();
    let y_max: u32 = dots.keys().map(|k| k.y).max().unwrap();
    for y in 0..=y_max {
        for x in 0..=x_max {
            if dots.contains_key(&Dot { x, y }) {
                print!("■");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn do_fold(fold: Fold, dots: &mut HashMap<Dot, bool>) {
    let mut dots_to_remove: Vec<Dot> = vec![];
    let mut dots_to_insert: Vec<Dot> = vec![];

    match fold.direction {
        Direction::X => {
            for dot in dots.keys() {
                if dot.x > fold.position {
                    let new_dot = Dot {
                        x: 2 * fold.position - dot.x,
                        y: dot.y,
                    };
                    dots_to_insert.push(new_dot);
                    dots_to_remove.push(dot.clone());
                }
            }
        }
        Direction::Y => {
            for dot in dots.keys() {
                if dot.y > fold.position {
                    let new_dot = Dot {
                        x: dot.x,
                        y: 2 * fold.position - dot.y,
                    };
                    dots_to_insert.push(new_dot);
                    dots_to_remove.push(dot.clone());
                }
            }
        }
    }

    for dot in dots_to_remove {
        dots.remove(&dot);
    }
    for dot in dots_to_insert {
        dots.insert(dot, true);
    }
}

fn parse_input(input: &str) -> (HashMap<Dot, bool>, Vec<Fold>) {
    let input_split: Vec<&str> = input.split("\n\n").collect();
    let dots_vec: Vec<Dot> = input_split[0].split('\n').map(get_dot).collect();
    let folds: Vec<Fold> = input_split[1]
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(get_fold)
        .collect();

    let mut dots: HashMap<Dot, bool> = HashMap::new();
    for dot in dots_vec {
        dots.insert(dot, true);
    }
    (dots, folds)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut dots, folds) = parse_input(&input);
    let fold = folds[0].clone();
    do_fold(fold, &mut dots);
    Some(dots.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut dots, folds) = parse_input(&input);
    for fold in folds {
        do_fold(fold, &mut dots);
    }
    display(&dots);
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(17));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(0));
    }
}
