use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;
use std::time::Instant;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let program: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse())
        .collect::<Result<Vec<Instruction>>>()?;

    let mut blocks = vec![];
    let mut block = vec![];
    for instr in &program {
        if &instr.op == "inp" {
            if block.len() != 0 {
                blocks.push(block);
            }
            block = vec![];
        }
        block.push(instr.clone())
    }
    blocks.push(block);

    let mut alu = ALU::new();

    let convert = |r: i64| -> String { r.to_string().chars().rev().collect() };

    let start = Instant::now();
    let digits = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut cache: HashMap<(i64, usize), Option<i64>> = HashMap::new();
    let result = find_model_number(&mut cache, &mut alu, &blocks, 0, digits);
    writeln!(io::stdout(), "Part1: {:?}, took: {:?}", convert(result.unwrap()), Instant::now() - start)?;

    let start = Instant::now();
    let mut cache: HashMap<(i64, usize), Option<i64>> = HashMap::new();
    let result = find_model_number_without_alu(&mut cache, 0, 0, digits);
    writeln!(io::stdout(), "Part1 with trimed func: {:?}, took: {:?}", convert(result.unwrap()), Instant::now() - start)?;

    let digits = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let start = Instant::now();
    let mut cache: HashMap<(i64, usize), Option<i64>> = HashMap::new();
    let result = find_model_number(&mut cache, &mut alu, &blocks, 0, digits);
    writeln!(io::stdout(), "Part2: {:?}, took: {:?}", convert(result.unwrap()), Instant::now() - start)?;

    let start = Instant::now();
    let mut cache: HashMap<(i64, usize), Option<i64>> = HashMap::new();
    let result = find_model_number_without_alu(&mut cache, 0, 0, digits);
    writeln!(io::stdout(), "Part2 with trimed func: {:?}, took: {:?}", convert(result.unwrap()), Instant::now() - start)?;

    Ok(())
}

fn find_model_number(
    cache: &mut HashMap<(i64, usize), Option<i64>>,
    alu: &mut ALU,
    blocks: &[Vec<Instruction>],
    index: usize,
    digits: [i64; 9],
) -> Option<i64> {
    // cache：HashMap实现的缓存，key是z的值和当前运行到的代码块，value是当前的输入数字的倒叙
    // alu：计算单元，含有四个变量，理论上可以用z代替，因为每次计算之后，实际上只有z的值是重要的，wxy的值都会在下一次运行被清零
    // blocks：所有的代码块
    // index：当前运行的代码块
    // digits：所有可能的数字排列

    // 首先保存当前的z值
    let z = alu.variables[3];
    if let Some(&answer) = cache.get(&(z, index)) {
        return answer;
    }

    for d in digits {
        // 修改alu的z值为上一个代码块运行后的z值
        alu.variables[3] = z;

        // 执行新的代码块
        alu.execute(&[d], &blocks[index]);

        // 记录新的z值为new_z
        let new_z = alu.variables[3];
        if index + 1 == blocks.len() {
            // index为13的时候，说明当前的输入已经达到了14位数字
            if new_z == 0 {
                // 假如这个时候的z值为0，实际上这个时候应该就是所要求的值了，不论是最大还是最小的
                // 但是注意这里返回的是第十四位的数字
                // 可以看出来，需要走到这里才能找到完整的输入数字
                // 将结果存入缓存/记忆
                cache.insert((new_z, index), Some(d));
                // 返回当前数字，用来拼接完整的数字
                return Some(d);
            }
            continue;
        }
        if let Some(best) = find_model_number(cache, alu, blocks, index + 1, digits) {
            // 找到下一个满足要求的数字
            // 实际上这里得到的应该是倒叙算出的数字
            // 将结果存入缓存/记忆
            cache.insert((new_z, index), Some(best * 10 + d));
            // best是计算是之后的输入数字，将best乘10再加上当前的数字
            // 就是截止目前为止的所有输入
            return Some(best * 10 + d);
        }
    }

    // 假如没有找到，直接返回None
    cache.insert((z, index), None);
    None
}

struct ALU {
    variables: [i64; 4], // [w, x, y, z]
}

impl ALU {
    fn new() -> Self {
        let variables = [0; 4];
        Self { variables }
    }

    fn clear(&mut self) {
        self.variables = [0; 4];
    }

    fn execute(&mut self, input: &[i64], program: &[Instruction]) {
        let mut input = input.iter();
        for instr in program {
            let output = match instr.ops[0] {
                Operand::Var(i) => i,
                Operand::Number(_) => unreachable!("Output must be a variable, {:?}", instr),
            };

            let op2 = if instr.op.as_str() != "inp" {
                match instr.ops[1] {
                    Operand::Var(i) => self.variables[i],
                    Operand::Number(i) => i,
                }
            } else {
                0
            };

            match instr.op.as_str() {
                "inp" => {
                    if let Some(&input_value) = input.next() {
                        self.variables[output] = input_value;
                    } else {
                        unreachable!("There is no input value for inp: {:?}", instr);
                    }
                }
                "add" => self.variables[output] += op2,
                "mul" => self.variables[output] *= op2,
                "div" => self.variables[output] /= op2,
                "mod" => self.variables[output] %= op2,
                "eql" => self.variables[output] = if self.variables[output] == op2 { 1 } else { 0 },
                _ => unreachable!("Wrong instruction: {:?}", instr),
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Var(usize),
    Number(i64),
}

#[derive(Debug, Clone)]
struct Instruction {
    op: String,
    ops: Vec<Operand>,
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        use Operand::*;

        let raw: Vec<&str> = s.trim().split(" ").collect();
        let (op, raw_ops) = raw.split_at(1);
        let op = op[0].to_string();
        let ops = raw_ops
            .iter()
            .map(|&o| match o {
                "w" => Var(0),
                "x" => Var(1),
                "y" => Var(2),
                "z" => Var(3),
                s => Number(s.parse().unwrap()),
            })
            .collect();
        Ok(Self { op, ops })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_alu() {
        use crate::{Instruction, ALU};
        let input = "inp w
        add z w
        mod z 2
        div w 2
        add y w
        mod y 2
        div w 2
        add x w
        mod x 2
        div w 2
        mod w 2";
        let program: Vec<Instruction> = input
            .lines()
            .map(|l| l.parse().unwrap())
            .collect::<Vec<Instruction>>();
        let mut alu = ALU::new();
        alu.execute(&[9], &program);
        assert_eq!(vec![1, 0, 0, 1], alu.variables);

        alu.clear();
        alu.execute(&[8], &program);
        assert_eq!(vec![1, 0, 0, 0], alu.variables);

        alu.clear();
        alu.execute(&[7], &program);
        assert_eq!(vec![0, 1, 1, 1], alu.variables);

        alu.clear();
        alu.execute(&[15], &program);
        assert_eq!(vec![1, 1, 1, 1], alu.variables);
        alu.clear();
    }
}

fn find_model_number_without_alu(
    cache: &mut HashMap<(i64, usize), Option<i64>>,
    z: i64,
    index: usize,
    digits: [i64; 9],
) -> Option<i64> {
    // cache：HashMap实现的缓存，key是z的值和当前运行到的代码块，value是当前的输入数字的倒叙
    // z：上一次的z值
    // index：当前运行的代码块
    // digits：所有可能的数字排列

    // 首先保存当前的z值
    if let Some(&answer) = cache.get(&(z, index)) {
        return answer;
    }

    for d in digits {
        // 计算新的z值
        let z = calc(d, z, index);

        if index == 13 {
            // index为13的时候，说明当前的输入已经达到了14位数字
            if z == 0 {
                // 假如这个时候的z值为0，实际上这个时候应该就是所要求的值了，不论是最大还是最小的
                // 但是注意这里返回的是第十四位的数字
                // 可以看出来，需要走到这里才能找到完整的输入数字
                // 将结果存入缓存/记忆
                cache.insert((z, index), Some(d));
                // 返回当前数字，用来拼接完整的数字
                return Some(d);
            }
            continue;
        }
        if let Some(best) = find_model_number_without_alu(cache, z, index + 1, digits) {
            // 找到下一个满足要求的数字
            // 实际上这里得到的应该是倒叙算出的数字
            // 将结果存入缓存/记忆
            cache.insert((z, index), Some(best * 10 + d));
            // best是计算是之后的输入数字，将best乘10再加上当前的数字
            // 就是截止目前为止的所有输入
            return Some(best * 10 + d);
        }
    }

    // 假如没有找到，直接返回None
    cache.insert((z, index), None);
    None
}

fn calc(w: i64, z: i64, index: usize) -> i64 {
    // div z [k]
    let k = [1, 1, 1, 1, 26, 1, 26, 26, 1, 26, 1, 26, 26, 26];
    // add x [p]
    let p = [10, 15, 14, 15, -8, 10, -16, -4, 11, -3, 12, -7, -15, -7];
    // add y [q]
    let q = [2, 16, 9, 0, 1, 12, 6, 6, 3, 5, 9, 3, 2, 3];
    let mut x = z % 26 + p[index];
    let mut z = z / k[index];
    x = if x == w { 0 } else { 1 };
    z = z * (25 * x + 1);
    z = z + (w + q[index]) * x;
    z
}

fn func(input: i64) {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut w = input;

    // x *= 0;
    // x += z;
    // x %= 26;
    x = z % 26;

    z /= 1; // 1, 1, 1, 1, 26, 1, 26, 26, 1, 26, 1, 26, 26, 26,
    x += 10; // 10, 15, 14, 15, -8, 10, -16, -4, 11, -3, 12, -7, -15, -7

    // x = if x == w { 1 } else { 0 };
    // x = if x == 0 { 1 } else { 0 };
    x = if x == w { 0 } else { 1 };

    // y *= 0;
    // y += 25;
    // y = 25;

    // y *= x;
    // y += 1;
    // z *= y;
    z = z * (25 * x + 1);
    // x 为 0 或 1
    // x == 0 => z = z
    // x == 1 => z = 26 * z

    // y *= 0;
    // y += w;
    // y = w;

    // y += 2; // k: 2, 16, 9, 0, 1, 12, 6, 6, 3, 5, 9, 3, 2, 3
    // y *= x;
    //     y = (w + 2) * x; // 2 => k

    // z += y;
    z = z + (w + 2) * x; // 2 => k

    // w + 2 不可能为负值
    // x 为 0 或 1
    // x == 0
}
