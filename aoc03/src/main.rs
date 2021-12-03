#![feature(drain_filter)]

use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let numbers: Vec<&[u8]> = input.lines().map(|s| s.as_bytes()).collect();
    part1(&numbers)?;
    part2(numbers.clone())?;
    Ok(())
}

fn part1(numbers: &[&[u8]]) -> Result<()> {
    let length = numbers[0].len();
    let mut result: Vec<i32> = vec![0; length];
    for number in numbers {
        for i in 0..length {
            match number[i] - '0' as u8 {
                0 => result[i] -= 1,
                1 => result[i] += 1,
                _ => return err!("wrong input not binary"),
            }
        }
    }
    for i in 0..length {
        if result[i] > 0 {
            result[i] = 1
        } else {
            result[i] = 0
        }
    }
    let result: Vec<u32> = result.iter().map(|&b| b as u32).collect();
    let gamma_rate: u32 = result.iter().fold(0, |acc, &b| (acc << 1) | b);
    let epsilon_rate: u32 = result.iter().fold(0, |acc, &b| (acc << 1) | (1 - b));
    writeln!(
        io::stdout(),
        "power consumption of the submarine: {}",
        gamma_rate * epsilon_rate
    )?;
    Ok(())
}

fn part2(numbers: Vec<&[u8]>) -> Result<()> {
    let length = numbers[0].len();

    let mut most_common = numbers.clone();
    for i in 0..length {
        let temp: Vec<&[u8]> = most_common
            .drain_filter(|b| b[i] == '1' as u8)
            .collect();
        if temp.len() >= most_common.len() {
            most_common = temp
        }
    }
    let oxygen_generator_rating = if most_common.len() == 1 {
        vec_to_u32(most_common[0])
    } else {
        return err!(
            "cannot find the oxygen generator rating, after search there remain {} rattings",
            most_common.len()
        );
    };

    let mut least_common = numbers.clone();
    for i in 0..length {
        if least_common.len() == 1 {
            break;
        }
        let temp: Vec<&[u8]> = least_common
            .drain_filter(|b| b[i] == '0' as u8)
            .collect();
        if temp.len() <= least_common.len() {
            least_common = temp
        }
    }
    let co2_scrubber_rating = if least_common.len() == 1 {
        vec_to_u32(least_common[0])
    } else {
        return err!(
            "cannot find the CO2 scrubber rating, after search there remain {} rattings",
            least_common.len()
        );
    };

    writeln!(
        io::stdout(),
        "life support rating of the submarine: {}",
        oxygen_generator_rating * co2_scrubber_rating
    )?;
    Ok(())
}

fn vec_to_u32(input: &[u8]) -> u32 {
    input.iter().fold(0, |acc, &b| acc << 1 | ((b - '0' as u8) as u32))
}
