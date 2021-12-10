use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const LEFT_CHARACTERS: [char; 4] = ['(', '[', '{', '<'];
const RIGHT_CHARACTERS: [char; 4] = [')', ']', '}', '>'];

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines: Vec<Remain> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .map(|chars| is_corrupted(&chars))
        .collect();
    part1(&lines)?;
    part2(&lines)?;
    Ok(())
}

fn part1(lines: &Vec<Remain>) -> Result<()> {
    writeln!(
        io::stdout(),
        "the total syntax error score for corrupted errors is {}",
        lines.iter().map(|l| illegal_char_to_score(l)).sum::<u32>()
    )?;
    Ok(())
}

fn part2(lines: &Vec<Remain>) -> Result<()> {
    let mut result: Vec<u64> = lines
        .iter()
        .map(|l| completion_line_to_score(l))
        .filter(|&i| i != 0)
        .collect();
    result.sort();
    writeln!(
        io::stdout(),
        "the middle score for completion string is {}",
        result[result.len() / 2]
    )?;

    Ok(())
}

#[derive(Debug)]
enum Remain {
    Char(char),
    List(Vec<char>),
    None,
}

fn is_corrupted(line: &[char]) -> Remain {
    let mut stack = vec![];
    for &c in line {
        if LEFT_CHARACTERS.contains(&c) {
            stack.push(c)
        } else if RIGHT_CHARACTERS.contains(&c) {
            let left = stack.pop().unwrap();
            if !is_matched_char(left, c) {
                return Remain::Char(c);
            }
        }
    }
    if stack.is_empty() {
        return Remain::None;
    } else {
        return Remain::List(stack);
    }
}

fn illegal_char_to_score(c: &Remain) -> u32 {
    match c {
        Remain::Char(c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
        _ => 0,
    }
}

fn completion_line_to_score(l: &Remain) -> u64 {
    match l {
        Remain::List(l) => l.iter().rev().fold(0, |acc, c| {
            acc * 5
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                }
        }),
        _ => 0,
    }
}

fn is_matched_char(left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') => true,
        ('[', ']') => true,
        ('{', '}') => true,
        ('<', '>') => true,
        _ => false,
    }
}
