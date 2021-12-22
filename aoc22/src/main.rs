use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::{Duration, Instant};

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

    let start = Instant::now();
    part1(&cuboids)?;
    writeln!(
        io::stdout(),
        "Part 1 took {:?} to computer",
        Instant::now() - start
    )?;
    let start = Instant::now();
    part2(&cuboids)?;
    writeln!(
        io::stdout(),
        "Part 2 with stack took {:?} to computer",
        Instant::now() - start
    )?;
    let start = Instant::now();
    part2_with_hashmap(&cuboids)?;
    writeln!(
        io::stdout(),
        "Part 2 with HashMap took {:?} to computer",
        Instant::now() - start
    )?;
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

fn part2_with_hashmap(cuboids: &Vec<Cuboid>) -> Result<()> {
    let result = calc_volume_with_hashmap(&cuboids);
    writeln!(io::stdout(), "Part2: there is {} cubes", result)?;
    Ok(())
}

fn calc_volume_with_hashmap(cuboids: &[Cuboid]) -> i64 {
    let mut counters: HashMap<Cuboid, i64> = HashMap::new(); // 初始化空表，用来存储每次变化之后所有的长方体和长方体出现的次数
    for next_cuboid in &cuboids[..] {
        let mut new_counters = counters.clone(); // 复制为新的HashMap，防止遍历HashMap的过程中对其进行修改，导致逻辑错误
        for (cuboid, count) in counters {
            // 遍历上一次的长方体
            if let Some(sub_cuboid) = cuboid.sub_cuboid(next_cuboid) {
                // 计算重叠区域
                // 重叠区域的次数为减去当前长方体的次数
                // 类似于利用栈实现的时候，新的长方体的状态为当前长方体的取反，具体见栈的实现的说明
                *new_counters.entry(sub_cuboid).or_insert(0) -= count;
            }
        }
        if next_cuboid.state {
            // 假如输入长方体状态为 on， 直接将表中的长方体的值加一，即出现次数加1
            *new_counters.entry(next_cuboid.clone()).or_insert(0) += 1;
        }
        counters = new_counters;
    }
    // 计算总体积的时候，要将长方体的体积乘上长方体出现的次数
    counters.iter().map(|(c, w)| c.volume() * w).sum()
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
                // if cuboid.state == next_cuboid.state {
                //     // 假如当前长方体和输入长方体的状态一致，重叠长方体的状态应该取反
                //     sub_cuboid.state = !next_cuboid.state;
                // } else {
                //     // 状态不一致时，重叠区域的状态应该和输入长方体的状态一致
                //     sub_cuboid.state = next_cuboid.state;
                // }
                // 仔细思考这两种情况，可以发现实际上重叠区域的状态，总是和当前长方体的状态相反。
                // 实际上可以在计算重复区域的时候，直接修改状态，减少需要修改的次数。
                // 但是在第一部分时，直接计算每一个输入和长方体(-50,-50,-50)、(50,50,50) 的重叠区域作为新的输入
                // 那么就会导致新的输入和旧的输入状态相反，所以可以不必改动。
                sub_cuboid.state = !cuboid.state;
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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
