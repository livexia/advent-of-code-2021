use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    
    let mut burrow: Vec<Vec<char>> = input.lines()
        .map(|l| l.chars().collect()).collect();
    writeln!(io::stdout(), "{}", input)?;

    let result = shortest_path(&burrow);
    writeln!(io::stdout(), "Part1: {}", result)?;

    burrow.insert(3, "  #D#C#B#A#  ".chars().collect());
    burrow.insert(4, "  #D#B#A#C#  ".chars().collect());
    
    for row in &burrow {
        writeln!(io::stdout(), "{}", row.iter().collect::<String>())?;
    }

    let result = shortest_path(&burrow);
    writeln!(io::stdout(), "Part2: {}", result)?;

    Ok(())
}

fn steps(burrow: &Vec<Vec<char>>) -> Vec<(usize, Vec<Vec<char>>)> {
   todo!()
}

fn is_orginized(burrow: &Vec<Vec<char>>) -> bool {
    burrow.iter().filter(|row| row[3..10].iter().collect::<String>() == "A#B#C#D").count() == burrow.len() - 3
}

fn shortest_path(burrow: &Vec<Vec<char>>) -> usize {
    todo!()
}