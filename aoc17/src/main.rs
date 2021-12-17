use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: String = input
        .chars()
        .filter(|&c| c.is_numeric() || c == '.' || c == ',' || c == '-')
        .collect();
    let mut input = input.split(",");
    let mut x_range = input
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse::<i32>().unwrap());
    let mut y_range = input
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse::<i32>().unwrap());
    let x_range = (x_range.next().unwrap(), x_range.next().unwrap());
    let y_range = (y_range.next().unwrap(), y_range.next().unwrap());

    let mut max_y_reaches = 0;
    let mut counter = 0;

    // 探针实际上在每一个单一方向上并不是匀变速直线运动，所以不能使用匀变速直线运动的规律来确定速度范围。

    // 当x方向上的速度减小到0时，还无法进入x的范围时，该初始速度无论如何无法达到目标范围
    // 也就是所在x方向上，对于每一个特定的初始速度，总有一个探针能在x方向上行进的最大距离
    // 当这个最大距离小于目标范围时，该初始速度一定无法使探针进入目标区域
    // 当x方向速度为0的时候，设x方向的初始速度为v，经过每一步速度减少1，那么总共经过v步速度为0
    // 那么实际上当x方向速度为0时，每一步经过的距离是公差为-1的等差数列
    // 所以x方向上探针经过的距离为 (v * v + v) / 2 设为s
    // 那么对于任意的v，假如s小于目标区域的最小x范围，该速度v无论如何无法达到目标范围
    // 实际上就是求解 (v * v + v) / 2 >= x_min 这样一个一元二次不等式
    // 当x方向上的速度大于x的最大范围时，1步之后，探针就超出了x的目标范围
    // 那么x方向上的最大速度为x的最大范围
    // 裁剪之后运行时间加快了10倍
    for v_x in (0..=x_range.1).filter(|v| (v * v + v) / 2 >= x_range.0) {
        // 因为初始位置不在区域范围内，所以至少需要1步 才能使探针在y方向的范围内
        // 无论探针初始速度是向上的还是向下的，因为目标区域在y=0之下
        // 那么最后总是需要以y向下的速度进入目标区域，而这个时候，每一步经过的距离实际上是以公差为1的等差数列
        // 所以在y向下的方向上，实际上不存在探针能探索的最大距离，与x方向不同。
        // 那么即使当y=0的时候，x方向的速度为0，且x在目标范围内，探针一定能进入目标区域。
        // 所以对于y方向，探针存在一个最大的同向、反向速度，但是不存在最小的同向、反向速度。
        // 事实上对于一个特定的x方向的初始速度，存在一个时间范围，在这个时间范围内，探针能保证探针的x位置是位于目标范围内
        // 那么理论上就可以根据这个时间范围和y的目标范围，推论出y的初始速度范围，需要增加运算复杂度，在此可以不看。

        // 考虑初始时y向下的大最大初始速度，也就是不论方向的最小初始速度
        // 设y方向的范围为y1..y2，且y1 < y2, y2 < 0
        // 那么当y方向的速度为 y1 - 1 的时候，只需要1步，探针就已经超出了y方向的范围
        // 所以y方向的最小初始速度(同方向的最大速度)是 y1

        // 考虑y方向的最大初始速度，这个时候探针应当是做向上的抛物线运动
        // 那么探针会两次经过 y = 0 的位置
        // 而第二次经过时探针y方向的速度应当与初始y方向的速度大小相同、方向相反，记为 v0'
        // 假设第二次经过 y = 0，之后至少需要1步才能进入目标范围
        // 那么当 v0' > y1 的时候只需要1步探针就超出了y方向的范围
        // 故 y 的最大初始速度(反方向最大速度)应该是 -v0，也就是 -y1

        // 可以看见实际上无论初始速度是向上还是向下的，最大的初始速度数值是一致的。

        for v_y in y_range.0..=-y_range.0 {
            let mut cur_pos = (0, 0);
            let init_v = (v_x, v_y);
            let mut cur_v = init_v;

            let mut max_y = 0;
            loop {
                max_y = max_y.max(cur_pos.1);
                match is_in_range(cur_pos.0, cur_pos.1, &x_range, &y_range) {
                    Position::Unknown => {
                        cur_pos = (next_pos(cur_pos.0, cur_v.0), next_pos(cur_pos.1, cur_v.1));
                        cur_v = (next_vx(cur_v.0), next_vy(cur_v.1));
                    }
                    Position::In => {
                        counter += 1;
                        max_y_reaches = max_y_reaches.max(max_y);
                        // writeln!(
                        //     io::stdout(),
                        //     "with init velocity {:?} can causes the probe within the target area",
                        //     init_v
                        // )?;
                        break;
                    }
                    Position::Out => {
                        // writeln!(io::stdout(), "with init velocity {:?} can not causes the probe within the target area", init_v)?;
                        break;
                    }
                }
            }
        }
    }
    writeln!(
        io::stdout(),
        "Part1: the highest y position it is {} it reaches on this trajectory",
        max_y_reaches
    )?;
    writeln!(io::stdout(), "Part2: there is {:} distinct initial velocity values cause the probe to be within the target area after any step", counter)?;
    Ok(())
}

fn next_pos(cur: i32, v: i32) -> i32 {
    cur + v
}

fn next_vx(cur: i32) -> i32 {
    if cur == 0 {
        cur
    } else if cur > 0 {
        cur - 1
    } else {
        cur + 1
    }
}

fn next_vy(cur: i32) -> i32 {
    cur - 1
}

fn is_in_range(x: i32, y: i32, x_range: &(i32, i32), y_range: &(i32, i32)) -> Position {
    if x > x_range.1 || y < y_range.0 {
        Position::Out
    } else if x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1 {
        Position::In
    } else {
        Position::Unknown
    }
    // x >= x_range.0 && x <= x_range.1 && y >= y_range.0 && y <= y_range.1
}

#[derive(Debug)]
enum Position {
    Unknown,
    In,
    Out,
}
