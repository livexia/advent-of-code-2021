use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = (usize, usize);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let heatmap: Vec<Vec<u8>> = input
        .lines()
        .map(|s| s.bytes().map(|b| b - '0' as u8).collect())
        .collect();
    let mut heatmap = HeatMap::new(heatmap);
    part1(&heatmap)?;
    part2(&mut heatmap)?;
    Ok(())
}

fn part1(heatmap: &HeatMap) -> Result<()> {
    writeln!(
        io::stdout(),
        "sum of the risk levels of all low points on your heightmap is {}",
        heatmap
            .low_points
            .iter()
            .fold(0, |acc, (i, j)| acc + 1 + heatmap.heatmap[*i][*j] as u32)
    )?;
    Ok(())
}

fn part2(heatmap: &mut HeatMap) -> Result<()> {
    let mut result = 1;
    for _ in 0..3 {
        if let Some(basin) = heatmap.basins.pop() {
            result *= basin;
        }
    };
    writeln!(
        io::stdout(),
        "multiply together the sizes of the three largest basins is {}",
        result
    )?;

    Ok(())
}

#[derive(Debug, Default)]
struct HeatMap {
    heatmap: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    visited: Vec<Vec<bool>>,
    low_points: Vec<Coord>,
    basins: BinaryHeap<usize>,
}

impl HeatMap {
    fn new(heatmap: Vec<Vec<u8>>) -> Self {
        let height = heatmap.len();
        let width = heatmap[0].len();
        let visited = vec![vec![false; width]; height];
        let mut heatmap = HeatMap {
            heatmap,
            height,
            width,
            visited,
            basins: BinaryHeap::with_capacity(3),
            ..Default::default()
        };
        heatmap.get_low_points();
        heatmap.find_all_basins();
        heatmap
    }

    fn up(&self, coord: Coord) -> Option<Coord> {
        if coord.0 > 0 {
            Some((coord.0 - 1, coord.1))
        } else {
            None
        }
    }

    fn down(&self, coord: Coord) -> Option<Coord> {
        if coord.0 + 1 < self.height {
            Some((coord.0 + 1, coord.1))
        } else {
            None
        }
    }

    fn left(&self, coord: Coord) -> Option<Coord> {
        if coord.1 > 0 {
            Some((coord.0, coord.1 - 1))
        } else {
            None
        }
    }

    fn right(&self, coord: Coord) -> Option<Coord> {
        if coord.1 + 1 < self.width {
            Some((coord.0, coord.1 + 1))
        } else {
            None
        }
    }

    fn adjacent_locations(&self, coord: Coord) -> Vec<Coord> {
        let mut result = vec![];
        if let Some(next) = self.up(coord) {
            result.push(next);
        }
        if let Some(next) = self.down(coord) {
            result.push(next);
        }
        if let Some(next) = self.left(coord) {
            result.push(next);
        }
        if let Some(next) = self.right(coord) {
            result.push(next);
        }
        result
    }

    fn get_low_points(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let cur_heat = self.heatmap[i][j];
                if self
                    .adjacent_locations((i, j))
                    .iter()
                    .all(|(next_i, next_j)| self.heatmap[*next_i][*next_j] > cur_heat)
                {
                    self.low_points.push((i, j));
                }
            }
        }
    }

    fn find_all_basins(&mut self) {
        for &point in &self.low_points {
            if self.visited[point.0][point.1] {
                continue;
            }
            let mut basin = HashSet::new();
            let mut stack = vec![point];
            while let Some((i, j)) = stack.pop() {
                self.visited[i][j] = true;
                basin.insert((i, j));
                let cur_heat = self.heatmap[i][j];
                stack.extend(self.adjacent_locations((i, j)).into_iter().filter(
                    |(next_i, next_j)| {
                        !self.visited[*next_i][*next_j]
                            && self.heatmap[*next_i][*next_j] != 9
                            && self.heatmap[*next_i][*next_j] > cur_heat
                    },
                ))
            }
            self.basins.push(basin.len());
        }
    }
}
