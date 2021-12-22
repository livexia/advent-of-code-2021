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

    // step (bool, (i64, i64), (i64, i64), (i64, i64))
    let cuboids: Vec<Cuboid> = input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Cuboid>>>()?;

    writeln!(io::stdout(), "there is {} steps", cuboids.len())?;

    part1(&cuboids)?;
    part2(&cuboids)?;
    Ok(())
}

fn part1(cuboids: &Vec<Cuboid>) -> Result<()> {
    let init_cuboid = Cuboid {
        state: true,
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    };
    let sub_cuboids = cuboids
        .iter()
        .filter_map(|c| c.sub_cuboid(&init_cuboid))
        .collect::<Vec<Cuboid>>();
    let result = calc_volume(&sub_cuboids);
    writeln!(
        io::stdout(),
        "Part1: ther is {} cubes are on the initialization procedure region",
        result
    )?;
    Ok(())
}

fn part2(cuboids: &Vec<Cuboid>) -> Result<()> {
    let result = calc_volume(&cuboids);
    writeln!(io::stdout(), "Part2: there is {} cubes", result)?;
    Ok(())
}

fn calc_volume(cuboids: &[Cuboid]) -> i64 {
    let mut stack: Vec<Cuboid> = vec![]; // 初始化空栈，用来存储每次变化之后所有的长方体
    for next_cuboid in &cuboids[..] {
        // 遍历每一次变化的长方体
        let mut new_stack = vec![]; // 建立新栈，防止在后续遍历对栈的直接修改，导致逻辑错误
        for cuboid in &stack {
            // 循环遍历栈中的长方体
            new_stack.push(cuboid.clone()); // 直接在新栈中存入当前的长方体
            if let Some(mut sub_cuboid) = cuboid.sub_cuboid(next_cuboid) {
                // 计算当前长方体和输入长方体的重叠区域
                // 防止累加两次重叠和减去两次重叠
                if cuboid.state == next_cuboid.state {
                    // 假如当前长方体和输入长方体的状态一致，重叠长方体的状态应该取反
                    sub_cuboid.state = !next_cuboid.state;
                } else {
                    // 状态不一致时，重叠区域的状态应该和输入长方体的状态一致
                    sub_cuboid.state = next_cuboid.state;
                }
                new_stack.push(sub_cuboid); // 把重叠区域的长方体放入栈中
            }
        }
        if next_cuboid.state {
            // 假如输入的长方体状态为打开，那么直接把输入推入栈中即可
            new_stack.push(next_cuboid.clone());
        }
        stack = new_stack; // 更新栈
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
struct Cuboid {
    state: bool,
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

impl Cuboid {
    fn volume(&self) -> i64 {
        (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
    }

    fn sub_cuboid(&self, other: &Cuboid) -> Option<Cuboid> {
        let x = Cuboid::sub_edge(self.x, other.x)?;
        let y = Cuboid::sub_edge(self.y, other.y)?;
        let z = Cuboid::sub_edge(self.z, other.z)?;
        Some(Cuboid {
            state: self.state,
            x,
            y,
            z,
        })
    }

    fn sub_edge((a, b): (i64, i64), (low, high): (i64, i64)) -> Option<(i64, i64)> {
        if a > high {
            // 假如一条线段的最小端大于另一条线段的最大端，则不存在重叠区域
            return None;
        }
        if b < low {
            // 假如一条线段的最大端小于另一条线段的最小端，则不存在重叠区域
            return None;
        }
        let low = low.max(a); // 重叠线段的最小端是，两条线段最小端中较大的那个
        let high = high.min(b); // 重叠线段的最大端是，两条线段最大端中较小的那个
        Some((low, high))
    }
}

impl FromStr for Cuboid {
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
