use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

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

    let mut cur = (0, 0);

    let mut t = HashSet::new();

    while !visited.iter().all(|v| v.iter().all(|&b| b)) {
        let risk = risk_from_start[cur.0][cur.1];
        for next in adjacent_coords(cur.0, cur.1, width) {
            if visited[next.0][next.1] {
                continue;
            }
            let risk = get_risk(next.0, next.1, &map) as u32 + risk;
            risk_from_start[next.0][next.1] = risk.min(risk_from_start[next.0][next.1]);
            t.insert(next);
        }

        let mut min_risk = u32::MAX;
        for next in t.clone() {
            let i = next.0;
            let j = next.1;
            if !visited[i][j] {
                if risk_from_start[i][j] < min_risk {
                    min_risk = risk_from_start[i][j];
                    cur = (i, j);
                }
            }
        }
        visited[cur.0][cur.1] = true;
        t.remove(&cur);
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
