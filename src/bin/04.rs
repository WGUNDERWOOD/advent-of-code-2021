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
#[derive(Clone)]
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

    fn has_won(&self) -> bool {

        // check rows
        for i in 0..DIM {
            let mut row_won = true;
            for j in 0..DIM {
                if !self.marked[i][j] {
                    row_won = false;
                }
            }
            if row_won {
                return true;
            }
        }

        // check columns
        for j in 0..DIM {
            let mut col_won = true;
            for i in 0..DIM {
                if !self.marked[i][j] {
                    col_won = false;
                }
            }
            if col_won {
                return true;
            }
        }
    false
    }

    fn get_sum_unmarked(&self) -> usize {
        let mut sum_unmarked = 0;

        for i in 0..DIM {
            for j in 0..DIM {
                if !self.marked[i][j] {
                    sum_unmarked += self.numbers[i][j];
                }
            }
        }
        sum_unmarked
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (draws, mut boards) = parse_input(input);
    let n_boards = boards.len();
    let mut won = false;
    let mut n = 0;
    let mut winning_draw = 0;
    let mut winning_board = Board{ numbers: vec![vec![0; DIM]; DIM],
                                   marked: vec![vec![false; DIM]; DIM] };

    while !won{
        let draw = draws[n];
        for b in 0..n_boards {
            let board = &mut boards[b];
            board.mark(draw);
            if board.has_won() {
                won = true;
                // TODO this clone is unnecessary
                winning_board = board.clone();
                winning_draw = draw;
            }
        }
        n += 1;
    }

    let sum_unmarked = winning_board.get_sum_unmarked();
    let score = sum_unmarked * winning_draw;
    Some(score.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (draws, mut boards) = parse_input(input);
    let n_boards = boards.len();
    let mut n = 0;
    let mut finished = false;
    let mut losing_draw = 0;
    let mut losing_board = Board{ numbers: vec![vec![0; DIM]; DIM],
                                  marked: vec![vec![false; DIM]; DIM] };

    while !finished{
        let draw = draws[n];
        losing_draw = draw;
        finished = true;
        for b in 0..n_boards {
            let board = &mut boards[b];
            board.mark(draw);
            if !board.has_won() {
                // TODO this clone is unnecessary
                losing_board = board.clone();
                finished = false;
            }
        }

        n += 1;
    }

    losing_board.mark(losing_draw);
    let sum_unmarked = losing_board.get_sum_unmarked();
    let score = sum_unmarked * losing_draw;
    Some(score.try_into().unwrap())
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
        assert_eq!(part_two(&input), Some(1924));
    }
}
