use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut cave_system = HashMap::new();
    input
        .lines()
        .map(|s| s.split("-").collect::<Vec<&str>>())
        .for_each(|path| {
            cave_system.entry(path[0]).or_insert(vec![]).push(path[1]);
            cave_system.entry(path[1]).or_insert(vec![]).push(path[0]);
        });

    part1(&cave_system)?;
    part2(&cave_system)?;
    Ok(())
}

fn part1(cave_system: &HashMap<&str, Vec<&str>>) -> Result<()> {
    let mut total_path = 0;
    let mut stack = vec![(vec![], &"start")];

    while let Some((visited, cur_cave)) = stack.pop() {
        if let Some(next_caves) = cave_system.get(cur_cave) {
            for cave in next_caves {
                let mut new_visited = visited.clone();
                if cave == &"end" {
                    total_path += 1;
                } else if cave == &"start" {
                    continue;
                } else {
                    if is_small(cave) && new_visited.contains(cave) {
                        continue;
                    }
                    new_visited.push(&cur_cave);
                    stack.push((new_visited, cave))
                }
            }
        }
    }
    writeln!(
        io::stdout(),
        "Part1: there is {} paths through this cave system",
        total_path
    )?;
    Ok(())
}

fn part2(cave_system: &HashMap<&str, Vec<&str>>) -> Result<()> {
    let mut total_path = 0;
    let mut stack = vec![(vec![], false, &"start")];

    while let Some((visited, small_cave, cur_cave)) = stack.pop() {
        if let Some(next_caves) = cave_system.get(cur_cave) {
            for cave in next_caves {
                let mut new_visited = visited.clone();
                let mut small_cave = small_cave;
                if cave == &"end" {
                    total_path += 1;
                } else if cave == &"start" {
                    continue;
                } else {
                    if is_small(cave) && new_visited.contains(cave) {
                        if small_cave {
                            continue;
                        } else {
                            small_cave = true
                        }
                    }
                    new_visited.push(&cur_cave);
                    stack.push((new_visited, small_cave, cave))
                }
            }
        }
    }
    writeln!(
        io::stdout(),
        "Part2: there is {} paths through this cave system",
        total_path
    )?;
    Ok(())
}

fn is_small(cave: &str) -> bool {
    cave.chars().nth(0).unwrap().is_lowercase()
}
