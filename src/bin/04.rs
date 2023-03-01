const DIM: usize = 5;

fn parse_input(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut ls: Vec<&str> = input.lines().collect();
    let draws: Vec<usize> = ls[0].split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect();

    ls.push("");
    let mut boards: Vec<Board> = vec![];
    let mut numbers: Vec<Vec<usize>> = vec![];

    for i in 2..ls.len() {
        if ls[i] == "" {
            let marked = vec![vec![false; DIM]; DIM];
            let mut board = Board{numbers: numbers.clone(), marked};
            board.numbers = numbers;
            boards.push(board);
            numbers = vec![]
        } else {
            let row: Vec<usize> = ls[i].split(" ").filter(|x| !x.is_empty())
                .map(|x| x.parse::<usize>().unwrap()).collect();
            numbers.push(row.clone());
        }
    }
    (draws, boards)
}

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<usize>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    fn mark(&mut self, n: usize) {
        for i in 0..DIM {
            for j in 0..DIM {
                if self.numbers[i][j] == n {
                    self.marked[i][j] = true
                }
            }
        }
    }

    // TODO this is not bingo
    fn has_won(&self) -> bool {
        let mut won = true;
        for i in 0..DIM {
            for j in 0..DIM {
                if !self.marked[i][j] {
                    won = false;
                }
            }
        }
    won
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (draws, mut boards) = parse_input(input);

    for n in draws.iter() {
        for board in &mut boards {
            board.mark(*n)
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(4512));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
