mod test;

use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

// 首先对字符串进行处理，将字符串转为数字，其中']'、'['、','，分别用-1，-2，-3来代表。
// 输出时则是将数字重新转为字符打印输出。

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<Vec<i32>> = input.lines().map(|s| convert(s)).collect();
    let result = input[1..]
        .iter()
        .fold(input[0].clone(), |acc, s| reduce(add(&acc, &s)));
    writeln!(
        io::stdout(),
        "Part1: the magnitude of the final sum is {}",
        calc_magnitude(&result)
    )?;

    let length = input.len();
    let mut result = 0;
    for i in 0..length {
        for j in (0..length).skip(i) {
            let s1 = &input[i];
            let s2 = &input[j];
            result = result.max(calc_magnitude(&reduce(add(s1, s2))));
        }
    };
    
    writeln!(
        io::stdout(),
        "Part2: the largest magnitude of any sum of two different snailfish numbers is {}", result
    )?;

    Ok(())
}

pub fn convert(s: &str) -> Vec<i32> {
    let mut result = vec![];
    for c in s.chars() {
        if c.is_whitespace() {
            continue;
        }
        match c {
            '[' => result.push(-1),
            ']' => result.push(-2),
            ',' => result.push(-3),
            c => result.push((c as u8 - '0' as u8) as i32),
        }
    }
    result
}

pub fn convert_back(s: &[i32]) -> String {
    let mut result = String::new();
    for &c in s {
        match c {
            -1 => result.push('['),
            -2 => result.push(']'),
            -3 => result.push(','),
            c => result.push_str(&c.to_string()),
        }
    }
    result
}

pub fn add(s1: &[i32], s2: &[i32]) -> Vec<i32> {
    let mut s = vec![];
    s.push(-1);
    s.extend_from_slice(s1);
    s.push(-3);
    s.extend_from_slice(s2);
    s.push(-2);
    s
}

pub fn height(s: &[i32]) -> i32 {
    // split and addition only add height 1
    let mut height = 0;
    let mut max_height = 0;
    for &c in s {
        if c == -1 {
            height += 1;
        } else if c == -2 {
            height -= 1;
        }
        max_height = max_height.max(height)
    }
    max_height
}

pub fn reduce(s: Vec<i32>) -> Vec<i32> {
    let mut result = s;
    loop {
        if height(&result) > 4 {
            result = explode(result);
        } else if result.iter().max().unwrap() > &9 {
            result = split(result);
        } else {
            break;
        }
    }
    result
}

pub fn explode(s: Vec<i32>) -> Vec<i32> {
    // 移除对最大高度的判断，实际上根据描述，当高度为四且存在大于9的节点时
    // 会进行一次split，split一次最多使深度加一
    // 而一次split之后，会进行explode操作，而explode操作确保深度在5之下
    // 深度达到4的时候，假如存在大于9的节点，才会进行新的一次split操作
    // 可见深度不可能达到6，除非连续进行了两次split，这也是为什么我这里原来有对最大高度的判断
    // 是我在对reduce中的逻辑有理解错误

    let mut pair = vec![];
    // let max_height = height(&s);
    let mut height = 0;
    let mut stack = s.clone();

    // 寻找需要explode的pair对，并不直接修改，记录位置
    for (i, c) in s.into_iter().enumerate() {
        if c == -1 {
            height += 1;
        } else if c == -2 {
            height -= 1;
        } else if c >= 0 {
            // if height >= max_height && pair.len() < 2 {
            if height > 4 && pair.len() < 2 {
                pair.push(i);
            }
            if pair.len() == 2 {
                break;
            }
        }
    }

    // 寻找并修改距离需要explode最近的pair的常规数字。
    // 注意在此并不对stack进进行删除操作，不影响记录的pair对位置。
    // 避免对位置的复杂计算，同时防止删除操作对后续值的影响。

    // 寻找并修改左侧最近的常规数字
    for i in (0..pair[0]).rev() {
        if stack[i] >= 0 {
            let value = stack[pair[0]] + stack[i];
            stack[i] = value;
            break;
        }
    }

    // 寻找并修改右侧最近的常规数字
    for i in pair[1] + 1..stack.len() {
        if stack[i] >= 0 {
            let value = stack[pair[1]] + stack[i];
            stack[i] = value;
            break;
        }
    }

    // 考虑每一次explode，考虑最内层次的情况，例如[x,y]。
    // [x,y] 必然位于一个pair中的左边或者右边。
    // explode后[x,y]会变成0，无论在左边还是右边有没有常规的数字。
    // 因为前面已经寻找了x和y可能的增加位置（左边和右边可能的最近的常规数字位置）。
    // 我们只需要把[x,y]中x的前一个位置也就是[改为0，即 stack[pair[1].0 - 1] = 0。
    // 从0到包含这个位置的所有字符是新的数字的开始。
    // 也是就是stack[..pair[0].0 + 1]或者stack[..=pair[0].0]
    // 新数字的剩余部分，就是从不包含y的后一个位置开始到结尾。
    // 也就是stack[pair[1].0 + 2..] 
    // 当然这只是其中一种参考方式，同样的也可以把y之后的一个位置也就是]置0；
    // 然后新的数字从0到x之前的一个位置为止，不包含x的前一个位置。
    // 剩余的数字则是从y之后的一个位置开始到结尾，包含y之后的一个位置。
    // stack[pair[1].0 + 1] = 0;
    // let start = pair[1].0 + 1;
    // let end = pair[0].0 - 1;

    stack[pair[0] - 1] = 0;
    let end = pair[0];
    let start = pair[1] + 2;

    // 根据取得的end位置和start位置对stack进行slice和concat操作，就得到了新的数字
    [&stack[..end], &stack[start..]].concat()
}

pub fn split(s: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut flag = false; // 记录split次数，保证只进行一次split
    for c in s {
        if c > 9 && !flag {
            stack.push(-1);
            stack.push(c / 2);
            stack.push(-3);
            stack.push(c - c / 2);
            stack.push(-2);
            flag = true;
        } else {
            stack.push(c);
        }
    }
    stack
}

pub fn calc_magnitude(s: &[i32]) -> i32 {
    // -1 => [
    // -3 => ,
    // -2 => ]
    // let mut result = 0;
    let mut stack = vec![];
    for &c in s.iter() {
        if c == -2 {
            let right = stack.pop().unwrap();
            let left = stack.pop().unwrap();
            let magnitude = left * 3 + right * 2;
            // result += magnitude;
            stack.push(magnitude);
        } else if c >= 0 {
            stack.push(c)
        }
    }
    stack[0]
}
