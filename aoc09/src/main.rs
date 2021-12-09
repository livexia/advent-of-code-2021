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

    let heatmap: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect())
        .collect();
    let height = heatmap.len();
    let width = heatmap[0].len();
    let highest_row = vec![10; width];
    let mut lowest_points: Vec<(usize, usize)> = vec![];

    let mut result = 0;
    for i in 0..height {
        for j in 0..width {
            let cur = &heatmap[i][j];
            if cur < &heatmap.get(i + 1).unwrap_or(&highest_row)[j]
                && cur
                    < &heatmap
                        .get(i.checked_sub(1).unwrap_or(height))
                        .unwrap_or(&highest_row)[j]
                && cur
                    < heatmap[i]
                        .get(j.checked_sub(1).unwrap_or(width))
                        .unwrap_or(&10)
                && cur < heatmap[i].get(j + 1).unwrap_or(&10)
            {
                result += *cur as u32 + 1;
                lowest_points.push((i, j));
            }
        }
    }

    writeln!(
        io::stdout(),
        "sum of the risk levels of all low points on your heightmap is {}",
        result
    )?;
    let mut basins: Vec<HashSet<(usize, usize)>> = vec![];
    for point in lowest_points {
        for basin in &basins {
            if basin.contains(&point) {
                continue;
            }
        }
        let mut basin = HashSet::new();
        let mut stack = vec![point];
        while let Some(cur_point) = stack.pop() {
            basin.insert(cur_point);
            let x = cur_point.0;
            let y = cur_point.1;
            let cur_heat = heatmap[x][y];
            if x >= 1 {
                let next_point = heatmap[x - 1][y];
                if next_point != 9 && next_point > cur_heat {
                    stack.push((x - 1, y))
                }
            }
            if y >= 1 {
                let next_point = heatmap[x][y - 1];
                if next_point != 9 && next_point > cur_heat {
                    stack.push((x, y - 1))
                }
            }
            if x + 1 < height {
                let next_point = heatmap[x + 1][y];
                if next_point != 9 && next_point > cur_heat {
                    stack.push((x + 1, y))
                }
            }
            if y + 1 < width {
                let next_point = heatmap[x][y + 1];
                if next_point != 9 && next_point > cur_heat {
                    stack.push((x, y + 1))
                }
            }
        }
        basins.push(basin);
    }
    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    writeln!(
        io::stdout(),
        "multiply together the sizes of the three largest basins is {}",
        &basins[..3].iter().fold(1, |acc, b| acc * b.len())
    )?;

    // part1()?;
    // part2()?;
    Ok(())
}
