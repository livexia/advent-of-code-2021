use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut energy_level: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect::<Vec<u8>>())
        .collect();
    let height = energy_level.len();
    let width = energy_level[0].len();

    let mut total_flashed: u32 = 0;
    let mut step = 0;
    loop {
        step += 1;
        let mut flashed = vec![vec![false; 10]; 10];
        let mut flashed_during_step = 0;
        let mut stack = vec![];
        for i in 0..height {
            for j in 0..width {
                if energy_level[i][j] == 9 {
                    energy_level[i][j] = 0;
                    stack.push((i, j));
                    flashed[i][j] = true;
                    flashed_during_step += 1;
                } else {
                    energy_level[i][j] += 1;
                }
            }
        }
        while let Some((i, j)) = stack.pop() {
            for (x, y) in get_adjacent(i, j, height, width) {
                if let Some(point) = flash_adjacent(&mut energy_level, &mut flashed, x, y) {
                    stack.push(point);
                    flashed_during_step += 1;
                }
            }
        }
        if step <= 100 {
            total_flashed += flashed_during_step;
            if step == 100 {
                writeln!(
                    io::stdout(),
                    "Part1: After {} steps there is total {} flashes",
                    step,
                    total_flashed
                )?;
            }
        }
        if flashed_during_step as usize == height * width {
            writeln!(
                io::stdout(),
                "Part2: The first step during which all octopuses flash is {}",
                step
            )?;
            break;
        }
    }
    Ok(())
}

fn get_adjacent(i: usize, j: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    if i >= height || j >= width {
        result
    } else {
        if i > 0 {
            result.push((i - 1, j))
        }
        if i + 1 < height {
            result.push((i + 1, j))
        }
        if j > 0 {
            result.push((i, j - 1))
        }
        if j + 1 < width {
            result.push((i, j + 1))
        }
        if i > 0 && j > 0 {
            result.push((i - 1, j - 1))
        }
        if i > 0 && j + 1 < width {
            result.push((i - 1, j + 1))
        }
        if i + 1 < height && j + 1 < width {
            result.push((i + 1, j + 1))
        }
        if i + 1 < height && j > 0 {
            result.push((i + 1, j - 1))
        }
        result
    }
}

fn flash_adjacent(
    energy_level: &mut Vec<Vec<u8>>,
    flashed: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
) -> Option<(usize, usize)> {
    if i >= energy_level.len() || j >= energy_level[0].len() {
        None
    } else if flashed[i][j] {
        None
    } else {
        if energy_level[i][j] == 9 {
            energy_level[i][j] = 0;
            flashed[i][j] = true;
            return Some((i, j));
        }
        energy_level[i][j] += 1;
        None
    }
}
