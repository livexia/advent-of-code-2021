use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (i64, i64, i64);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // step (bool, (i64, i64), (i64, i64), (i64, i64))
    let cubes: Vec<Cube> = input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Cube>>>()?;

    writeln!(io::stdout(), "there is {} steps", cubes.len())?;

    part1(&cubes)?;
    part2(&cubes)?;
    Ok(())
}

fn part1(cubes: &Vec<Cube>) -> Result<()> {
    let init_cube = Cube {
        state: true,
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    };
    let sub_cubes = cubes
        .iter()
        .filter_map(|c| c.sub_cube(&init_cube))
        .collect::<Vec<Cube>>();
    let result = calc_volume(&sub_cubes);
    writeln!(
        io::stdout(),
        "Part1: ther is {} cubes are on the initialization procedure region",
        result
    )?;
    Ok(())
}

fn part2(cubes: &Vec<Cube>) -> Result<()> {
    let result = calc_volume(&cubes);
    writeln!(io::stdout(), "Part2: there is {} cubes", result)?;
    Ok(())
}

fn calc_volume(cubes: &[Cube]) -> i64 {
    let mut stack = vec![cubes[0].clone()];
    for next_cube in &cubes[1..] {
        let mut new_stack = vec![];
        for cube in &stack {
            new_stack.push(cube.clone());
            if let Some(mut sub_cube) = cube.sub_cube(next_cube) {
                if cube.state == next_cube.state {
                    sub_cube.state = !next_cube.state;
                } else {
                    sub_cube.state = next_cube.state;
                }
                new_stack.push(sub_cube);
            }
        }
        if next_cube.state {
            new_stack.push(next_cube.clone());
        }
        stack = new_stack;
        println!("{}", stack.len());
    }
    stack
        .iter()
        .map(|c| {
            let sign = if c.state { 1 } else { -1 };
            c.volume() * sign
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Cube {
    state: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cube {
    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn sub_cube(&self, other: &Cube) -> Option<Cube> {
        let x = Cube::sub_edge(self.x, other.x)?;
        let y = Cube::sub_edge(self.y, other.y)?;
        let z = Cube::sub_edge(self.z, other.z)?;
        Some(Cube {
            state: self.state,
            x,
            y,
            z,
        })
    }

    fn sub_edge((a, b): (i64, i64), (low, high): (i64, i64)) -> Option<(i64, i64)> {
        if a > high {
            return None;
        }
        if b < low {
            return None;
        }
        let low = low.max(a);
        let high = high.min(b);
        Some((low, high))
    }
}

impl FromStr for Cube {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let state = if s.starts_with("on") { true } else { false };
        let ranges: Vec<&str> = s.split(" ").last().unwrap().split(",").collect();

        let range: Vec<&str> = ranges[0][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let x = (range[0], range[1]);

        let range: Vec<&str> = ranges[1][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let y = (range[0], range[1]);

        let range: Vec<&str> = ranges[2][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let z = (range[0], range[1]);

        Ok(Self { state, x, y, z })
    }
}
