use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut commands: Vec<(&str, u32)> = vec![];
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        if line.len() == 2 {
            let direction = line[0];
            let length: u32 = line[1].parse().unwrap();
            commands.push((direction, length));
        } else {
            return err!("input error");
        }
    }

    part1(&commands)?;
    part2(&commands)?;
    Ok(())
}

fn part1(commands: &[(&str, u32)]) -> Result<()> {
    let mut vertical = 0;
    let mut horizontal = 0;
    for (direction, length) in commands {
        match (direction, length) {
            (&"down", s) => vertical += s,
            (&"up", s) => vertical -= s,
            (&"forward", s) => horizontal += s,
            _ => return err!("commands error")
        };
    }
    writeln!(io::stdout(), "horizontal * depth: {}", vertical * horizontal)?;
    Ok(())
}

fn part2(commands: &[(&str, u32)]) -> Result<()> {
    let mut aim = 0;
    let mut vertical = 0;
    let mut horizontal = 0;
    for (direction, length) in commands {
        match (direction, length) {
            (&"down", s) => aim += s,
            (&"up", s) => aim -= s,
            (&"forward", s) => {
                horizontal += s;
                vertical += aim * s;
            },
            _ => return err!("commands error")
        };
    }
    writeln!(io::stdout(), "horizontal * depth: {}", vertical * horizontal)?;
    Ok(())
}