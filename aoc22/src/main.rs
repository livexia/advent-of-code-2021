use std::error::Error;
use std::io::{self, Read, Write};
use std::collections::HashSet;
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Coord = (i64, i64, i64);

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    // step (bool, (i64, i64), (i64, i64), (i64, i64))
    let steps: Vec<Step> = input.lines()
        .map(|s| s.parse()).collect::<Result<Vec<Step>>>()?;

    writeln!(io::stdout(), "there is {} steps", steps.len())?;

    part1(&steps)?;
    part2_brute_force(&steps)?;
    // part2(&steps)?;
    Ok(())
}

fn part2(steps: &Vec<Step>) -> Result<()> {
    // the step inside the region x=-50..50,y=-50..50,z=-50..50
    // those step do not overlap with other steps

    let mut count = 0;

    let start = 0;

    let mut stacks = vec![steps[start].clone()];

    for other in &steps[start + 1..] {
        let mut new_stacks = vec![];
        for step in &stacks {
            let i1 = step.intersection_with(other);
            let i2 = other.intersection_with(step);
            if i1.len() == 0 && i2.len() == 0 {
                new_stacks.push(step.clone());
                new_stacks.push(other.clone());
            } else if i1.len() == 1 && i2.len() == 1 {
                if !step.state && !other.state {
                    new_stacks.push(step.clone())
                } else {
                    let state = if step.state && other.state {
                        false
                    } else if step.state && !other.state {
                        false
                    } else if !step.state && other.state {
                        true
                    } else {
                        unreachable!("Should be unreachable")
                    };
                    let p1 = i1[0];
                    let p2 = i2[0];
                    let new_step = Step {
                        state,
                        min_x: p1.0.min(p2.0),
                        max_x: p1.0.max(p2.0),
                        min_y: p1.1.min(p2.1),
                        max_y: p1.1.max(p2.1),
                        min_z: p1.2.min(p2.2),
                        max_z: p1.2.max(p2.2),
                    };
                    new_stacks.push(step.clone());
                    new_stacks.push(new_step);
                    if other.state {
                        new_stacks.push(other.clone());
                    }
                }
            } else if i1.len() == 2 && i2.len() == 0 {
    
            } else if i1.len() == 0 && i2.len() == 2 {
    
            } else {
                return err!("Existence of situations not considered, {}, {}", i1.len(), i2.len());
            }
            println!("{}, {}", i1.len(), i2.len());
        }
        stacks = new_stacks;
    }
    println!("{:?}", stacks);
    let mut count = 0;
    for step in stacks {
        let sign = if step.state {
            1
        } else {
            -1
        };
        count += sign * step.count();
    }
    println!("{:?}", count);
    Ok(())
}


fn part2_brute_force(steps: &Vec<Step>) -> Result<()> {
    let mut on_cubes: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut off_cubes: HashSet<(i64, i64, i64)> = HashSet::new();
    for step in steps.iter().rev() {
        for x in step.min_x..=step.max_x {
            for y in step.min_y..=step.max_y {
                for z in step.min_z..=step.max_z {
                    if on_cubes.contains(&(x, y, z)) || off_cubes.contains(&(x, y, z)) {
                        continue;
                    }
                    if step.state {
                        on_cubes.insert((x, y, z));
                    } else {
                        off_cubes.insert((x, y, z));
                    }
                }
            }
        }
    }
    writeln!(io::stdout(), "Part2: {}", on_cubes.len())?;
    Ok(())
}

fn part1(steps: &Vec<Step>) -> Result<()> {
    let steps: Vec<Step> = steps.iter().map(|s| s.part1_step()).collect();
    let mut on_cubes: HashSet<(i64, i64, i64)> = HashSet::new();
    for step in steps {
        let next_state = step.state;
        for x in step.min_x..=step.max_x {
            for y in step.min_y..=step.max_y {
                for z in step.min_z..=step.max_z {
                    let last_state = on_cubes.contains(&(x, y, z));
                    if next_state {
                        on_cubes.insert((x, y, z));
                    }
                    if last_state && !next_state {
                        on_cubes.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    writeln!(io::stdout(), "Part1: ther is {} cubes are on the initialization procedure region", on_cubes.len())?;
    Ok(())
}


#[derive(Debug, Clone)]
struct Step {
    state: bool,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Step {
    fn count(&self) -> i64 {
        (self.max_x - self.min_x + 1)
            * (self.max_y - self.min_y + 1)
            * (self.max_z - self.min_z + 1)
    }

    fn contains(&self, coord: &Coord) -> bool {
        let x= coord.0;
        let y= coord.1;
        let z= coord.2;
        x >= self.min_x && x <= self.max_x
            && y >= self.min_y && y <= self.max_y
            && z >= self.min_z && z <= self.max_z
    }

    fn intersection_with(&self, other: &Step) -> Vec<Coord> {
        // 8
        let mut intersections = vec![];
        for vertex in self.vertexs() {
            if other.contains(&vertex) {
                intersections.push(vertex);
            }
        }
        intersections
    }

    fn vertexs(&self) -> [Coord; 8]{
        [
            (self.min_x, self.min_y, self.min_z),
            (self.min_x, self.min_y, self.max_z),
            (self.min_x, self.max_y, self.min_z),
            (self.min_x, self.max_y, self.max_z),
            (self.max_x, self.min_y, self.min_z),
            (self.max_x, self.min_y, self.max_z),
            (self.max_x, self.max_y, self.min_z),
            (self.max_x, self.max_y, self.max_z),
        ]
    }

    fn part1_step(&self) -> Self {
        let min_x = if self.min_x < -50 {
            -50
        } else {
            self.min_x
        };
        let max_x = if self.max_x > 50 {
            50
        } else {
            self.max_x
        };
        let min_y = if self.min_y < -50 {
            -50
        } else {
            self.min_y
        };
        let max_y = if self.max_y > 50 {
            50
        } else {
            self.max_y
        };
        let min_z = if self.min_z < -50 {
            -50
        } else {
            self.min_z
        };
        let max_z = if self.max_z > 50 {
            50
        } else {
            self.max_z
        };

        Self { state: self.state , min_x, max_x, min_y, max_y, min_z, max_z }
    }
}

impl FromStr for Step {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let state = if s.starts_with("on") {
            true
        } else {
            false
        };
        let ranges: Vec<&str> = s.split(" ").last().unwrap().split(",").collect();
        
        let range: Vec<&str> = ranges[0][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let (min_x, max_x) = (range[0], range[1]);
    
        let range: Vec<&str> = ranges[1][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let (min_y, max_y) = (range[0], range[1]);
    
        let range: Vec<&str> = ranges[2][2..].split("..").collect();
        let range: Vec<i64> = range.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let (min_z, max_z) = (range[0], range[1]);
    
        Ok(Self { state, min_x, max_x, min_y, max_y, min_z, max_z })
    }
}