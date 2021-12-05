use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::mem::swap;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = i32;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut entries: Vec<Entry> = vec![];
    for line in input.lines() {
        entries.push(line.parse()?)
    }
    writeln!(io::stdout(), "there is {} entries", entries.len())?;

    part1(&entries)?;
    part2(&entries)?;
    Ok(())
}

fn part1(entries: &[Entry]) -> Result<()> {
    let mut grid: HashMap<Point, u32> = HashMap::new();
    for entry in entries {
        for point in entry.points_on_the_horizontal_and_vertical_line() {
            *grid.entry(point).or_default() += 1;
        }
    }
    let result = grid
        .into_iter()
        .fold(0, |acc, (_, v)| if v > 1 { acc + 1 } else { acc });

    writeln!(
        io::stdout(),
        "there is {} points do at least two lines overlap",
        result
    )?;
    Ok(())
}

fn part2(entries: &[Entry]) -> Result<()> {
    let mut grid: HashMap<Point, u32> = HashMap::new();
    for entry in entries {
        for point in entry.all_point_on_the_line() {
            *grid.entry(point).or_default() += 1;
        }
    }
    let result = grid
        .into_iter()
        .fold(0, |acc, (_, v)| if v > 1 { acc + 1 } else { acc });

    writeln!(
        io::stdout(),
        "there is {} points do at least two lines overlap",
        result
    )?;
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: Coord,
    y: Coord,
}

impl Point {
    fn new(x: Coord, y: Coord) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let raw_point: Vec<&str> = s.trim().split(",").collect();
        if raw_point.len() != 2 {
            return err!("Wrong point: {}", s);
        }
        Ok(Point {
            x: raw_point[0].parse()?,
            y: raw_point[1].parse()?,
        })
    }
}

#[derive(Debug, Clone)]
struct Entry {
    start: Point,
    end: Point,
}

impl Entry {
    fn all_point_on_the_line(&self) -> Vec<Point> {
        let mut result = self.points_on_the_diagonal_line();
        result.extend(self.points_on_the_horizontal_and_vertical_line());
        result
    }
    fn points_on_the_horizontal_and_vertical_line(&self) -> Vec<Point> {
        let start = &self.start;
        let end = &self.end;
        let mut result = vec![];
        if self.is_horizontal_and_vertical_line() {
            if start.x == end.x {
                (start.y..=end.y).for_each(|j| result.push(Point::new(start.x, j)));
            } else {
                (start.x..=end.x).for_each(|i| result.push(Point::new(i, start.y)));
            }
        }
        result
    }

    fn points_on_the_diagonal_line(&self) -> Vec<Point> {
        let start = &self.start;
        let end = &self.end;
        let mut result = vec![];
        if self.is_diagonal_line() {
            let k = (start.x - end.x) / (start.y - end.y);
            let b = start.y - k * start.x;
            (start.x..=end.x).for_each(|i| result.push(Point::new(i, k * i + b)));
        }
        result
    }

    fn is_horizontal_and_vertical_line(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn is_diagonal_line(&self) -> bool {
        !self.is_horizontal_and_vertical_line()
            && ((self.start.x - self.end.x) / (self.start.y - self.end.y)).abs() == 1
    }
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let raw_entry: Vec<&str> = s.split(" -> ").collect();
        if raw_entry.len() != 2 {
            return err!("Wrong entry: {}", s);
        }
        let mut start: Point = raw_entry[0].parse()?;
        let mut end: Point = raw_entry[1].parse()?;
        if start.x > end.x {
            swap(&mut start, &mut end)
        } else if start.x == end.x {
            if start.y > end.y {
                swap(&mut start, &mut end)
            }
        }
        Ok(Entry { start, end })
    }
}
