use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanners: Vec<Vec<Coord>> = vec![];
    let mut scanner: Vec<Coord> = vec![];
    for line in input.lines() {
        if line.contains(",") {
            scanner.push(line.parse()?)
        } else if line.trim().len() == 0 {
            scanners.push(scanner);
            scanner = vec![];
        }
    }
    scanners.push(scanner);

    let length = scanners.len();

    let mut known_scanner = HashSet::new();
    known_scanner.insert(0);

    let mut stack = vec![0];
    let mut scanners_dis = vec![Coord::new(0, 0, 0)];

    while let Some(i) = stack.pop() {
        for j in 1..length {
            if known_scanner.contains(&j) {
                continue;
            } else {
                if let Some(scanner) = get_scanner_coord(&mut scanners, i, j) {
                    writeln!(
                        io::stdout(),
                        "{} -> {}, coord is {:?}(relative to scanner 0)",
                        i,
                        j,
                        scanner
                    )?;
                    scanners_dis.push(scanner);
                    known_scanner.insert(j);
                    stack.push(j);
                }
            }
        }
        if known_scanner.len() == length {
            break;
        }
    }
    if known_scanner.len() != scanners.len() {
        writeln!(
            io::stdout(),
            "Error: only {} scanners found",
            known_scanner.len()
        )?;
    }
    let beacons: HashSet<Coord> = scanners.clone().into_iter().flatten().collect();
    writeln!(io::stdout(), "Part1: there is {} beacons", beacons.len())?;

    let mut max_dis = 0;
    for s1 in scanners_dis.iter() {
        for s2 in scanners_dis.iter() {
            max_dis = max_dis.max(s1.dis(s2));
        }
    }
    writeln!(
        io::stdout(),
        "Part2: the largest Manhattan distance between any two scanners is {}",
        max_dis
    )?;

    Ok(())
}

fn get_scanner_coord(scanners: &mut [Vec<Coord>], i: usize, j: usize) -> Option<Coord> {
    let beacons0 = &scanners[i];
    let beacons1 = &scanners[j];
    let all_rotates: Vec<Vec<Coord>> = beacons1.iter().map(|s| s.rotate()).collect();
    let rotates_number = all_rotates[0].len();

    for i in 0..rotates_number {
        let mut counter1: HashMap<Coord, i32> = HashMap::new();
        for coord1 in beacons0 {
            for coords in &all_rotates {
                let sub_coord = coord1.sub(&coords[i]);
                let counter = counter1.entry(sub_coord.clone()).or_insert(0);
                *counter += 1;
                if counter >= &mut 12 {
                    let scanner = sub_coord;
                    for index in 0..beacons1.len() {
                        scanners[j][index] = scanner.add(&all_rotates[index][i]);
                    }
                    return Some(scanner);
                }
            }
        }
    }
    None
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn from_vec(v: &[i32]) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    fn sub(&self, point: &Self) -> Self {
        Self {
            x: self.x - point.x,
            y: self.y - point.y,
            z: self.z - point.z,
        }
    }

    fn dis(&self, point: &Self) -> i32 {
        (self.x - point.x).abs() + (self.y - point.y).abs() + (self.z - point.z).abs()
    }

    fn add(&self, point: &Self) -> Self {
        Self {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }

    fn rotate(&self) -> Vec<Self> {
        fn all_rotate(coord: Vec<i32>) -> Vec<Vec<i32>> {
            if coord.len() == 1 {
                vec![vec![-1 * coord[0]], coord]
            } else {
                let mut output: Vec<Vec<i32>> = vec![];
                for sign in [-1, 1] {
                    for (index, first) in coord.iter().enumerate() {
                        let mut remain = coord.clone();
                        remain.remove(index);
                        for mut permutation in all_rotate(remain) {
                            permutation.insert(0, sign * first.clone());
                            output.push(permutation);
                        }
                    }
                }
                output
            }
        }

        all_rotate(vec![self.x, self.y, self.z])
            .iter()
            .map(|v| Self::from_vec(v))
            .collect()
    }
}

impl FromStr for Coord {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let raw: Vec<&str> = s.split(",").collect();
        Ok(Self {
            x: raw[0].parse().unwrap(),
            y: raw[1].parse().unwrap(),
            z: raw[2].parse().unwrap(),
        })
    }
}
