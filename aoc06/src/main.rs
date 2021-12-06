use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut lanternfishes: Vec<i64> = vec![0; 9];
    input.trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .for_each(|i: usize| lanternfishes[i] += 1);

    lanternfish_after_day(0, lanternfishes.clone())?;
    lanternfish_after_day(80, lanternfishes.clone())?;
    lanternfish_after_day(256, lanternfishes.clone())?;

    Ok(())
}

fn lanternfish_after_day(day: u32, mut lanternfishes: Vec<i64>) -> Result<()> {
    for _ in 0..day {
        lanternfishes[7] += lanternfishes[0];
        lanternfishes.rotate_left(1);

        // let zero = lanternfishes[0];
        // for i in 0..9 {
        //     if i != 0 {
        //         lanternfishes[i - 1] = lanternfishes[i];
        //     }
        //     lanternfishes[i] = 0;
        // }
        // lanternfishes[6] += zero;
        // lanternfishes[8] += zero;
    }
    writeln!(
        io::stdout(),
        "After {} days there is {} lanternfishes.",
        day,
        lanternfishes.iter().sum::<i64>()
    )?;
    Ok(())
}
