mod test;

use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Int = u8;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&&input)?;
    part2(&&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut input: Vec<Option<Box<Number>>> = input
        .lines()
        .map(|s| Number::from_str(s))
        .collect::<Result<Vec<Option<Box<Number>>>>>()?;

    let first = input.remove(0);
    let result = input.into_iter().fold(first, |acc, s| {
        let mut acc = Number::addition(acc, s);
        acc.as_mut().unwrap().reduce();
        acc
    });
    writeln!(io::stdout(), "Part1: the magnitude of the final sum is {}", result.as_ref().unwrap().calc_magnitude())?;
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let input: Vec<Option<Box<Number>>> = input
        .lines()
        .map(|s| Number::from_str(s))
        .collect::<Result<Vec<Option<Box<Number>>>>>()?;

    let mut max_magnitude = 0;
    for i in 0..input.len() {
        for j in 1+1..input.len() {
            let mut result = Number::addition(input[i].clone(), input[j].clone());
            result.as_mut().unwrap().reduce();
            max_magnitude = max_magnitude.max(result.as_ref().unwrap().calc_magnitude())
        }
    }
    
    writeln!(io::stdout(), "Part2: the largest magnitude of any sum of two different snailfish numbers is {}", max_magnitude)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Number {
    value: Option<Int>,
    left: Option<Box<Number>>,
    right: Option<Box<Number>>,
    height: usize,
}

impl Number {
    fn new(
        value: Option<Int>,
        mut left: Option<Box<Number>>,
        mut right: Option<Box<Number>>,
    ) -> Self {
        if let Some(left) = left.as_mut() {
            left.height += 1;
            left.update_height();
        }
        if let Some(right) = right.as_mut() {
            right.height += 1;
            right.update_height();
        }
        Number {
            value,
            left,
            right,
            height: 0,
        }
    }
    fn with_value(value: Int) -> Self {
        Number {
            value: Some(value),
            left: None,
            right: None,
            height: 0,
        }
    }

    fn add_child(&mut self, child: Option<Box<Number>>) {
        if self.left.is_none() {
            self.left = child
        } else {
            self.right = child
        }
    }

    fn from_str(s: &str) -> Result<Option<Box<Self>>> {
        // [1, 2]
        let mut stack: Vec<u8> = s
            .bytes()
            .filter(|c| !c.is_ascii_whitespace())
            .rev()
            .collect();
        stack.pop();
        Ok(Number::from_stack(&mut stack))
    }

    fn from_stack(stack: &mut Vec<u8>) -> Option<Box<Self>> {
        let mut number = Number::new(None, None, None);
        while let Some(b) = stack.pop() {
            if b == ']' as u8 {
                break;
            } else if b == ',' as u8 {
                continue;
            } else if b == '[' as u8 {
                number.add_child(Number::from_stack(stack));
            } else {
                number.add_child(Some(Box::new(Number::with_value(b - '0' as u8))));
            }
        }
        number.update_height();
        Some(Box::new(number))
    }

    fn addition(left: Option<Box<Number>>, right: Option<Box<Number>>) -> Option<Box<Number>> {
        Some(Box::new(Number::new(None, left, right)))
    }

    fn reduce(&mut self) {
        let mut reduced = false;
        while !reduced {
            let result = self.explode(None, None, false);
            if result.2 {
                reduced = false;
                continue;
            }
            reduced = !self.split();
        }
    }

    fn calc_magnitude(&self) -> i32 {
        if let Some(value) = self.value {
            return value as i32;
        }
        let mut result = 0;
        result += 3 * self.left.as_ref().unwrap().calc_magnitude();
        result += 2 * self.right.as_ref().unwrap().calc_magnitude();

        result
    }

    fn find_right<'a>(
        child: &'a mut Option<Box<Number>>,
        leftmost: Option<&'a mut Option<Box<Number>>>,
        right_value: Option<u8>,
        exploded: bool,
    ) -> (Option<&'a mut Option<Box<Number>>>, Option<u8>, bool) {
        let value = right_value.clone();
        if child.as_ref().unwrap().value.is_some() {
            if let Some(value) = value {
                let height = child.as_ref().unwrap().height;
                *child = Some(Box::new(Number::with_value(
                    child.as_ref().unwrap().value.unwrap() + value,
                )));
                child.as_mut().unwrap().height = height;
                (leftmost, None, exploded)
            } else {
                (Some(child), right_value, exploded)
            }
        } else {
            child
                .as_mut()
                .unwrap()
                .explode(leftmost, right_value, exploded)
        }
    }

    fn explode<'a>(
        &'a mut self,
        leftmost: Option<&'a mut Option<Box<Number>>>,
        right_value: Option<u8>,
        exploded: bool,
    ) -> (Option<&'a mut Option<Box<Number>>>, Option<u8>, bool) {
        if self.height > 3 && !exploded {
            let right_value = self.right.take().unwrap().value;
            self.value = Some(0);
            if let Some(left_value) = self.left.take().unwrap().value {
                if let Some(Some(leftmost)) = leftmost {
                    if let Some(value) = leftmost.value {
                        leftmost.value = Some(value + left_value);
                    }
                }
            }
            (None, right_value, true)
        } else {
            let (leftmost, right_value, exploded) =
                Number::find_right(&mut self.left, leftmost, right_value, exploded);
            let (leftmost, right_value, exploded) =
                Number::find_right(&mut self.right, leftmost, right_value, exploded);
            (leftmost, right_value, exploded)
        }
    }

    fn split(&mut self) -> bool {
        if let Some(value) = self.value {
            if value > 9 {
                let left_value = value / 2;
                self.add_child(Some(Box::new(Number::with_value(left_value))));
                self.add_child(Some(Box::new(Number::with_value(value - left_value))));
                self.value = None;
                self.update_height();
                return true;
            }
            false
        } else {
            if self.left.as_mut().unwrap().split() {
                return true;
            }
            if self.right.as_mut().unwrap().split() {
                return true;
            }
            false
        }
    }

    fn update_height(&mut self) {
        if let Some(left) = self.left.as_mut() {
            left.height = self.height + 1;
            left.update_height();
        }
        if let Some(right) = self.right.as_mut() {
            right.height = self.height + 1;
            right.update_height();
        }
    }

    fn output(&self) -> String {
        let mut result = String::new();
        if let Some(value) = self.value {
            result.push_str(&value.to_string());
        } else {
            result.push('[');
            result.push_str(&self.left.as_deref().unwrap().output());
            result.push(',');
            result.push_str(&self.right.as_deref().unwrap().output());
            result.push(']');
        }

        result
    }
}
