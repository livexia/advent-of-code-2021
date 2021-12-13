use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut coordinates = vec![];
    let mut instructions = vec![];
    for line in input.lines() {
        if line.contains(",") {
            let coord = line
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<i32>>();
            coordinates.push((coord[0], coord[1]))
        } else if line.starts_with("fold along ") {
            let instr: Vec<&str> = line.split("=").collect();
            if instr[0].ends_with("x") {
                instructions.push((instr[1].parse::<i32>().unwrap(), 0))
            } else if instr[0].ends_with("y") {
                instructions.push((0, instr[1].parse::<i32>().unwrap()))
            }
        }
    }
    writeln!(
        io::stdout(),
        "there is {} coordinates and {} instructions",
        coordinates.len(),
        instructions.len()
    )?;

    let mut stack = coordinates;

    for (i, (x1, y1)) in instructions.into_iter().enumerate() {
        let mut next_stack = vec![];
        for (x2, y2) in stack.into_iter() {
            let mut new_x = x2;
            let mut new_y = y2;
            if x1 == 0 && y2 > y1 {
                new_y = y1 * 2 - y2;
            } else if y1 == 0 && x2 > x1 {
                new_x = x1 * 2 - x2;
            }
            next_stack.push((new_x, new_y));
        }
        stack = next_stack;
        if i == 0 {
            writeln!(
                io::stdout(),
                "Part1: there is {} visible dots after {}th fold",
                stack.len(),
                i + 1
            )?;
        }
    }

    let mut result = String::new();
    let max_x = stack.iter().map(|(x, _)| x).max().unwrap();
    let max_y = stack.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if stack.contains(&(x, y)) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
        result.push('\n');
    }
    writeln!(io::stdout(), "Part2: \n{}", result)?;
    Ok(())
}
