use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let depths: Vec<u32> = input.lines()
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&depths)?;
    part2(&depths)?;
    Ok(())
}

fn part1(depths: &[u32]) -> Result<()> {
    let mut result = 0;
    for i in 0..depths.len() - 1 {
        if depths[i] < depths[i + 1] {
            result += 1
        }
    }
    writeln!(io::stdout(), "there are {} measurements that are larger than the previous measurement.", result)?;
    Ok(())
}

fn part2(depths: &[u32]) -> Result<()> {
    let mut result = 0;
    for i in 0..depths.len() - 3 {
        if depths[i] < depths[i + 3] {
            result += 1
        }
    }
    writeln!(io::stdout(), "there are {} sums that are larger than the previous sum..", result)?;
    Ok(())
}
