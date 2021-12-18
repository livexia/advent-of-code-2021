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

    // let number = Number::from_str("[3,[1,2]]")?;
    // println!("{:#?}", number);

    // let number = Number::from_str("[[1,2],3]")?;
    // println!("{:#?}", number);

    // let number = Number::from_str("[1,2]")?;
    // println!("{:#?}", number);

    // let number = Number::from_str("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")?;
    // println!("{:#?}", number);

    // let numbers: Vec<Option<Box<Number>>> = input
    //     .lines()
    //     .map(|s| Number::from_str(s))
    //     .collect::<Result<_>>()?;

    // println!("there is {} snailfish numbers", numbers.len());

    // let mut number = Number::with_value(11);
    // println!("{:#?}", number);
    // number.split();
    // println!("{:#?}", number);

    // let number = Number::from_str("[3,[1,2]]")?;
    // let number = Number::from_str("[[[[[9,8],1],2],3],4]");
    // println!("{:#?}", number);

    // let number1 = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]] ")?;
    // let number2 = Number::from_str("[1,1]")?;
    // println!("{:#?}", Number::addition(number1, number2));

    let number = Number::from_str("[[[[[9,8],1],2],3],4]")?;
    number.unwrap().reduce();

    // part1()?;
    // part2()?;
    Ok(())
}

#[derive(Debug)]
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
        let mut stack: Vec<u8> = s.bytes().filter(|c| !c.is_ascii_whitespace()).rev().collect();
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

    fn addition(left: Option<Box<Number>>, right: Option<Box<Number>>) -> Number {
        Number::new(None, left, right)
    }

    fn reduce(mut self) -> (Option<Box<Self>>, Option<(Int, Int)>) {
        let mut values = None;
        let number: Option<Box<Self>>;
        if self.height < 4 {
            if let Some(left) = self.left.take() {
                let result = left.reduce();
                self.left = result.0;
                values = result.1;
                if let Some(values) = values {
                    if let Some(mut right) = self.right.take() {
                        if right.value.is_some() {
                            *right.value.as_mut().unwrap() += values.1;
                        }
                    }
                }
            }
            number = Some(Box::new(self))
        } else {
            number = None;
            values = Some(self.explode());
        }
        // if let Some(value) = self.value {
        //     if value > 9 {
        //         self.split()
        //     }
        // }
        return (number, values);
    }

    fn explode(&self) -> (Int, Int) {
        let left_value = self.left.as_ref().unwrap().value.unwrap();
        let right_value = self.right.as_ref().unwrap().value.unwrap();
        return (left_value, right_value)
    }

    fn split(&mut self) {
        if let Some(value) = self.value {
            let left_value = value / 2;
            self.add_child(Some(Box::new(Number::with_value(left_value))));
            self.add_child(Some(Box::new(Number::with_value(value - left_value))));
            self.value = None;
        }
        self.update_height();
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
}
