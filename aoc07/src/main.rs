use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut horizontal_positions: HashMap<i32, i32> = HashMap::new();
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|p| *horizontal_positions.entry(p).or_insert(0) += 1);

    // part1()?;
    let burn_rate = |step: i32| -> i32 { step };
    let min_fuel = fuel_usage(&horizontal_positions, &burn_rate);
    println!("part1: must spend {} fuel.", min_fuel);

    // part2()?;
    let burn_rate = |step: i32| -> i32 { (1 + step) * step / 2};
    let min_fuel = fuel_usage(&horizontal_positions, &burn_rate);
    println!("part2: must spend {} fuel.", min_fuel);
    Ok(())
}

fn fuel_usage(positions: &HashMap<i32, i32>, burn_rate: &dyn Fn(i32) -> i32) -> i32 {
    let &max_position = positions.keys().max().unwrap();
    let &min_position = positions.keys().min().unwrap();

    let mut min_fuel = i32::MAX;
    for p in min_position..=max_position {
        let cur_fuel: i32 = positions
            .iter()
            .map(|(k, v)| burn_rate((k - p).abs()) * v)
            .sum();
        if cur_fuel < min_fuel {
            min_fuel = cur_fuel;
        }
    }
    min_fuel
}
