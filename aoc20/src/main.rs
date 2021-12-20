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

    let input: Vec<&str> = input.lines().collect();

    let algo: Vec<char> = input[0].chars().collect();

    assert_eq!(512, algo.len());

    let mut light_set: HashSet<(i32, i32)> = HashSet::new();

    let mut min_h = 0;
    let mut min_w = 0;
    let mut max_h = input.len() as i32 - 2;
    let mut max_w = 0;

    for (i, line) in input[2..].iter().enumerate() {
        max_w = max_w.max(line.len() as i32);
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                light_set.insert((i as i32, j as i32));
            }
        }
    }
    // output_map(&light_set, min_h, max_h, min_w, max_w)?;

    let flip_flag = algo[0] == '#' && algo[511] == '.';

    let mut void_lit = false;

    let steps = 50;
    for i in 1..=steps {
        let offset = 2; // apply the image enhancement algorithm 1 times, only affects 2 around

        let mut new_light_set: HashSet<(i32, i32)> = HashSet::new();

        for i in min_h - offset..max_h + offset {
            for j in min_w - offset..max_w + offset {
                let mut index = 0;
                let mut multi = 1;
                for coord in near_pixels(i, j).iter().rev() {
                    if light_set.contains(&coord) {
                        index += 1 * multi;
                    } else if void_lit && is_in_void(coord.0, coord.1, min_h, max_h, min_w, max_w) {
                        index += 1 * multi;
                    }
                    multi *= 2;
                }

                if algo[index] == '#' {
                    new_light_set.insert((i, j));
                }
            }
        }
        light_set = new_light_set;

        // output_map(&light_set, min_h, max_h, min_w, max_w)?;
        if i == 2 || i == 50 {
            writeln!(io:: stdout(), "There is {} pixels are lit in the resulting image after apply the image enhancement algorithm {} times.", light_set.len(), i)?;
        }

        if flip_flag {
            void_lit = !void_lit;
        }

        min_h -= offset;
        max_h += offset;
        min_w -= offset;
        max_w += offset;

        if i % 2 == 0 {
            for i in (max_h - 2 * offset..max_h).rev() {
                if (min_w..max_w).any(|j| light_set.contains(&(i, j))) {
                    continue;
                } else {
                    max_h -= 1;
                }
            }
    
            for i in min_h..=min_h + 2 * offset {
                if (min_w..max_w).any(|j| light_set.contains(&(i, j))) {
                    continue;
                } else {
                    min_h += 1;
                }
            }
            for j in (max_w - 2 * offset..max_w).rev() {
                if (min_h..max_h).any(|i| light_set.contains(&(i, j))) {
                    continue;
                } else {
                    max_w -= 1;
                }
            }
            for j in min_w..=min_w + 2 * offset {
                if (min_h..max_h).any(|i| light_set.contains(&(i, j))) {
                    continue;
                } else {
                    min_w += 1;
                }
            }
        }
    }

    Ok(())
}

fn output_map(
    light_set: &HashSet<(i32, i32)>,
    min_h: i32,
    max_h: i32,
    min_w: i32,
    max_w: i32,
) -> Result<()> {
    let mut s = String::new();
    for i in min_h..max_h {
        for j in min_w..max_w {
            if light_set.contains(&(i, j)) {
                s.push('#')
            } else {
                s.push('.')
            }
        }
        s.push('\n');
    }
    s.push('\n');
    writeln!(io::stdout(), "{}", s)?;
    Ok(())
}

fn near_pixels(i: i32, j: i32) -> Vec<(i32, i32)> {
    vec![
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
}

fn is_in_void(i: i32, j: i32, min_h: i32, max_h: i32, min_w: i32, max_w: i32) -> bool {
    i >= max_h || i < min_h || j >= max_w || j < min_w
}
