use std::borrow::Borrow;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Number = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut numbers: Vec<Number> = vec![];
    let mut board: Vec<Vec<Number>> = vec![];
    let mut boards: Vec<Board> = vec![];

    for line in input.lines() {
        if line.contains(",") {
            numbers = line
                .split(',')
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect();
        } else if line.trim() == "" {
            if board.len() == 0 {
                continue;
            }
            boards.push(Board::new(board)?);
            board = vec![];
        } else {
            board.push(
                line.split_whitespace()
                    .into_iter()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            );
        }
    }
    boards.push(Board::new(board)?);
    writeln!(io::stdout(), "there is {} numbers and {} boards", numbers.len(), boards.len())?;

    part1(numbers.clone(), boards.clone())?;
    part2(numbers.clone(), boards.clone())?;
    Ok(())
}

fn part1(numbers: Vec<Number>, mut boards: Vec<Board>) -> Result<()> {
    for n in numbers {
        for board in &mut boards {
            board.mark(n);
            if board.is_bingo() {
                writeln!(io::stdout(), "first win board final score: {}", board.sum() * n)?;
                return Ok(());
            }
        }
    }
    Ok(())
}

fn part2(numbers: Vec<Number>, mut boards: Vec<Board>) -> Result<()> {
    let mut final_score = 0;
    for n in numbers {
        for board in &mut boards {
            if !board.is_bingo() {
                board.mark(n);
                if board.is_bingo() {
                    final_score = board.sum() * n;
                }
            }
        }
    }
    writeln!(io::stdout(), "last win board's final score: {}", final_score)?;
    Ok(())
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<Vec<Number>>,
    sum: Number,
}

impl Board {
    fn new(board: Vec<Vec<Number>>) -> Result<Self> {
        if board.len() == 5 {
            for row in board.iter() {
                if row.len() != 5 {
                    return err!("board is not 5 * 5");
                }
            }
            let sum: Number = board.iter().map(|r| r.iter().sum::<Number>()).sum();
            Ok(Board { board, sum })
        } else {
            err!("board is not 5 * 5, {:?}", board)
        }
    }

    fn is_bingo(&self) -> bool {
        (0..5).map(|r| self.sum_row_at(r)).any(|s| s == -5)
            || (0..5).map(|c| self.sum_col_at(c)).any(|s| s == -5)
    }

    fn sum_row_at(&self, r: usize) -> Number {
        self.board[r].iter().sum()
    }

    fn sum_col_at(&self, c: usize) -> Number {
        (0..5).map(|i| self.board[i][c]).sum()
    }

    fn sum(&self) -> Number {
        self.sum
    }

    fn mark(&mut self, n: Number) {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == n {
                    self.board[i][j] = -1;
                    self.sum -= n;
                }
            }
        }
    }
}
