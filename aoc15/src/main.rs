use std::error::Error;
use std::io::{self, Read, Write};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect())
        .collect();
    let width = map.len();
    writeln!(
        io::stdout(),
        "The map is {}*{}, {}",
        width,
        width,
        width == map[0].len()
    )?;

    writeln!(
        io::stdout(),
        "Part1: the lowest total risk is {}",
        lowest_risk(&map, width)
    )?;
    writeln!(
        io::stdout(),
        "Part2: the lowest total risk is {}",
        lowest_risk(&map, width * 5)
    )?;


    Ok(())
}

fn lowest_risk(map: &Vec<Vec<u8>>, width: usize) -> u32 {
    let mut visited = vec![vec![false; width]; width];
    visited[0][0] = true;

    let mut risk_from_start = vec![vec![u32::MAX; width]; width];
    risk_from_start[0][0] = 0;

    let mut t = BinaryHeap::new();
    t.push(Reverse((0, (0, 0))));
    while let Some(Reverse((risk ,(i, j)))) = t.pop() {
        for (x, y) in adjacent_coords(i, j, width) {
            if visited[x][y] {
                continue;
            }
            visited[x][y] = true;
            let risk = risk_from_start[x][y].min(get_risk(x, y, &map) as u32 + risk);
            risk_from_start[x][y] = risk;
            t.push(Reverse((risk, (x, y))))
        }
    }
    risk_from_start[width - 1][width - 1]
}

fn adjacent_coords(i: usize, j: usize, width: usize) -> Vec<(usize, usize)> {
    let mut coords = vec![];
    if i + 1 < width {
        coords.push((i + 1, j))
    }
    if j + 1 < width {
        coords.push((i, j + 1))
    }
    if j > 1 {
        coords.push((i, j - 1))
    }
    if i > 1 {
        coords.push((i - 1, j))
    }

    coords
}

fn get_risk(i: usize, j: usize, map: &Vec<Vec<u8>>) -> u8 {
    let width = map.len();
    let offset_i = i / width;
    let offset_j = j / width;
    let value = map[i % width][j % width] + (offset_i + offset_j) as u8;
    if value > 9 {
        value - 9
    } else {
        value
    }
}
