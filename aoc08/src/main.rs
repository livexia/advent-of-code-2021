use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Read, Write};
use std::str::FromStr;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

const ONE: usize = 2;
const FOUR: usize = 4;
const SEVEN: usize = 3;
const EIGHT: usize = 7;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = input
        .lines()
        .map(|s| s.parse())
        .collect::<Result<Vec<Entry>>>()?;

    part1(&entries)?;
    part2(&entries)?;
    Ok(())
}

fn part1(entries: &[Entry]) -> Result<()> {
    let mut result = 0;
    for entry in entries {
        for s in &entry.output {
            match s.len() {
                ONE | FOUR | SEVEN | EIGHT => result += 1,
                _ => (),
            }
        }
    }
    writeln!(io::stdout(), "digits 1, 4, 7, or 8 appear {} times", result)?;
    Ok(())
}

fn part2(entries: &[Entry]) -> Result<()> {
    let result: i32 = entries.iter().map(|e| e.get_output()).sum();
    writeln!(io::stdout(), "sum of the output values is {}", result)?;
    Ok(())
}

#[derive(Debug, Default)]
struct Entry {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
    zero: Vec<char>,
    one: Vec<char>,
    two: Vec<char>,
    three: Vec<char>,
    four: Vec<char>,
    five: Vec<char>,
    six: Vec<char>,
    seven: Vec<char>,
    eight: Vec<char>,
    nine: Vec<char>
}

impl Entry {
    fn new(input: Vec<Vec<char>>, output: Vec<Vec<char>>) -> Self {
        let mut entry = Entry {
            input,
            output,
            ..Default::default()
        };

        entry.figure_the_pattern();

        entry
    }

    fn find_easy_digits(&self, len: usize) -> Option<Vec<char>> {
        self.output
            .clone()
            .into_iter()
            .chain(self.input.clone().into_iter())
            .find(|s| s.len() == len)
    }

    fn find_digits_with_length(&self, len: usize) -> HashSet<Vec<char>> {
        self.output
            .clone()
            .into_iter()
            .chain(self.input.clone().into_iter())
            .filter(|s| s.len() == len)
            .collect()
    }

    fn figure_the_pattern(&mut self) {
        let mut length_five_digits = self.find_digits_with_length(5);
        let mut length_six_digits = self.find_digits_with_length(6);

        self.one = self.find_easy_digits(ONE).unwrap().clone();
        self.four = self.find_easy_digits(FOUR).unwrap().clone();
        self.seven = self.find_easy_digits(SEVEN).unwrap().clone();
        self.eight = self.find_easy_digits(EIGHT).unwrap().clone();

        let one: HashSet<&char> = self.one.iter().collect();
        // find three
        for digit in &length_five_digits {
            let digit_set = digit.iter().collect();
            if one.is_subset(&digit_set) {
                self.three = digit.clone();
            }
        }
        length_five_digits.remove(&self.three);

        // find six
        for digit in &length_six_digits {
            let digit_set = digit.iter().collect();
            if !one.is_subset(&digit_set) {
                self.six = digit.clone();
            }
        }
        length_six_digits.remove(&self.six);


        let six: HashSet<&char> = self.six.iter().collect();
        // find zero and nine
        for digit in &length_six_digits {
            let digit_set: HashSet<&char> = digit.iter().collect();
            for x in six.difference(&digit_set) {
                if self.four.contains(x) {
                    self.zero = digit.clone();
                } else {
                    self.nine = digit.clone();
                }
            }
        }

        // find five and two
        for digit in &length_five_digits {
            let digit_set: HashSet<&char> = digit.iter().collect();
            if digit_set.difference(&six).count() == 0 {
                self.five = digit.clone();
            } else {
                self.two = digit.clone();
            }
        }
    }

    fn get_output(&self) -> i32 {
        let mut s = String::new();
        for o in &self.output {
            if o == &self.zero {
                s.push('0')
            } else if o == &self.one {
                s.push('1')
            } else if o == &self.two {
                s.push('2')
            } else if o == &self.three {
                s.push('3')
            } else if o == &self.four {
                s.push('4')
            } else if o == &self.five {
                s.push('5')
            } else if o == &self.six {
                s.push('6')
            } else if o == &self.seven {
                s.push('7')
            } else if o == &self.eight {
                s.push('8')
            } else if o == &self.nine {
                s.push('9')
            }
        }
        s.parse().unwrap()
    }
}

impl FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        let split: Vec<&str> = s.split("|").collect();
        if split.len() != 2 {
            return err!("Wrong signal pattern: {}", s);
        }
        let mut input: Vec<Vec<char>> = split[0]
            .split_whitespace()
            .map(|s| s.trim().chars().collect())
            .collect();
        input.iter_mut().for_each(|s| s.sort());

        let mut output: Vec<Vec<char>> = split[1]
            .split_whitespace()
            .map(|s| s.trim().chars().collect())
            .collect();
        output.iter_mut().for_each(|s| s.sort());

        Ok(Self::new(input, output))
    }
}
