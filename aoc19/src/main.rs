use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::vec;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Coord = Vec<i32>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut scanners: Vec<Vec<Coord>> = vec![];
    let mut scanner = vec![];
    for line in input.lines() {
        if line.contains(",") {
            scanner.push(line.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
        } else if line.trim().len() == 0 {
            scanners.push(scanner.clone());
            scanner = vec![];
        }
    }
    scanners.push(scanner);

    let length = scanners.len();

    assert!(scanners.iter().all(|v| v.iter().all(|v1| v1.len() == 3)));

    let mut not_overlaps = HashSet::new();

    let mut known_scanner = HashSet::new();
    known_scanner.insert(0);

    let mut stack = vec![0];
    let mut scanners_dis = vec![vec![0, 0, 0]];

    while let Some(i) = stack.pop() {
        for j in 1..length {
            if !known_scanner.contains(&j) && !not_overlaps.contains(&(i, j)) {
                let result = get_scanner_coord(&scanners[i], &scanners[j]);
                if result.0.len() == 3 {
                    let scanner = result.0;
                    scanners[j] = result.1;
                    writeln!(
                        io::stdout(),
                        "{} -> {}, found scanner {}, coord is {:?}(relative to scanner 0)",
                        i,
                        j,
                        j,
                        scanner
                    )?;
                    scanners_dis.push(scanner);
                    known_scanner.insert(j);
                    stack.push(j);
                } else {
                    not_overlaps.insert((i, j));
                }
            }
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
            max_dis =
                max_dis.max((s1[0] - s2[0]).abs() + (s1[1] - s2[1]).abs() + (s1[2] - s2[2]).abs());
        }
    }
    writeln!(
        io::stdout(),
        "Part2: the largest Manhattan distance between any two scanners is {}",
        max_dis
    )?;

    Ok(())
}

fn get_scanner_coord(beacons0: &[Coord], beacons1: &[Coord]) -> (Coord, Vec<Coord>) {
    let all_rotates: Vec<Vec<Coord>> = beacons1.iter().map(|s| all_rotate(s.to_vec())).collect();
    let all_rotates_number = all_rotates[0].len();
    let mut scanner = vec![];
    let mut beacons = vec![];

    for i in 0..all_rotates_number {
        let mut counter1: HashMap<Vec<i32>, i32> = HashMap::new();
        for coord1 in beacons0 {
            for coords in &all_rotates {
                let dis_coord = vec![
                    coord1[0] - coords[i][0],
                    coord1[1] - coords[i][1],
                    coord1[2] - coords[i][2],
                ];
                *counter1.entry(dis_coord).or_insert(0) += 1;
            }
        }
        let max = counter1.iter().max_by_key(|v| v.1).unwrap();

        if max.1 >= &12 {
            for &c in max.0 {
                scanner.push(c);
            }
            for coords in &all_rotates {
                let coord = vec![
                    coords[i][0] + scanner[0],
                    coords[i][1] + scanner[1],
                    coords[i][2] + scanner[2],
                ];
                beacons.push(coord);
            }
            break;
        }
    }
    (scanner, beacons)
}

fn all_rotate(coord: Coord) -> Vec<Coord> {
    if coord.len() == 1 {
        vec![vec![-1 * coord[0]], coord]
        // vec![coord]
    } else {
        let mut output: Vec<Coord> = vec![];
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
