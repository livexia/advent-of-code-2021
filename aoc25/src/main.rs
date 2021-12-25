use std::error::Error;
use std::io::{self, Read, Write};
use std::collections::HashSet;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut east_sea_cucumbers = HashSet::new();
    let mut south_sea_cucumbers = HashSet::new();

    let south_bound = input.lines().count();
    let mut east_bound = 0;
    for (i, line) in input.lines().enumerate() {
        east_bound = line.len();
        for (j, c) in line.char_indices() {
            match c {
                '.' => continue,
                'v' => south_sea_cucumbers.insert((i, j)),
                '>' => east_sea_cucumbers.insert((i, j)),
                _ => unreachable!("Wrong input char {} at {:?}", c, (i, j)),
            };
        }
    }



    let mut steps = 0;
    let mut move_counter = 1;

    while move_counter > 0 {
        steps += 1;
        move_counter = 0;
        let mut next_east_sea_cucumbers = HashSet::new();
        for &(i, j) in &east_sea_cucumbers {
            let next_j = next(j, east_bound);
            if east_sea_cucumbers.contains(&(i, next_j)) || south_sea_cucumbers.contains(&(i, next_j)) {
                next_east_sea_cucumbers.insert((i, j));
            } else {
                move_counter += 1;
                next_east_sea_cucumbers.insert((i, next_j));
            }
        }
        east_sea_cucumbers = next_east_sea_cucumbers;
        let mut next_south_sea_cucumbers = HashSet::new();
        for &(i, j) in &south_sea_cucumbers {
            let next_i = next(i, south_bound);
            if east_sea_cucumbers.contains(&(next_i, j)) || south_sea_cucumbers.contains(&(next_i, j)) {
                next_south_sea_cucumbers.insert((i, j));
            } else {
                move_counter += 1;
                next_south_sea_cucumbers.insert((next_i, j));
            }
        }
        south_sea_cucumbers = next_south_sea_cucumbers;
    }

    writeln!(io::stdout(), "{}", steps)?;

    Ok(())
}

fn next(cur: usize, bound: usize) -> usize {
    if cur + 1 < bound {
        cur + 1
    } else {
        0
    }
}

// the sea cucumbers in the east-facing herd attempt to move forward one location
// the sea cucumbers in the south-facing herd attempt to move forward one location
// sea cucumbers that move off the right edge of the map appear on the left edge
// sea cucumbers that move off the bottom edge of the map appear on the top edge
// always check whether their destination location is empty before moving, even if that destination is on the opposite side of the map
// sea cucumbers need to stop moving