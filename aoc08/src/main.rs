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
    let result: u32 = entries.iter().map(|e| e.get_output()).sum();
    writeln!(io::stdout(), "sum of the output values is {}", result)?;
    Ok(())
}

#[derive(Debug, Default)]
struct Entry {
    input: Vec<String>,
    output: Vec<String>,
    pattern: Vec<String>,
    query_map: HashMap<String, u32>,
}

impl Entry {
    fn new(input: Vec<String>, output: Vec<String>) -> Self {
        let mut entry = Entry {
            input,
            output,
            pattern: vec!["0".to_string(); 10],
            ..Default::default()
        };

        entry.figure_the_pattern();

        entry
    }

    fn find_easy_digits(&self, len: usize) -> Option<String> {
        self.output
            .clone()
            .into_iter()
            .chain(self.input.clone().into_iter())
            .find(|s| s.len() == len)
    }

    fn find_digits_with_length(&self, len: usize) -> HashSet<String> {
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

        self.pattern[1] = self.find_easy_digits(ONE).unwrap().clone();
        self.pattern[4] = self.find_easy_digits(FOUR).unwrap().clone();
        self.pattern[7] = self.find_easy_digits(SEVEN).unwrap().clone();
        self.pattern[8] = self.find_easy_digits(EIGHT).unwrap().clone();

        let one: HashSet<char> = self.pattern[1].chars().collect();
        // find three
        for digit in &length_five_digits {
            let digit_set = digit.chars().collect();
            if one.is_subset(&digit_set) {
                self.pattern[3] = digit.clone();
            }
        }
        length_five_digits.remove(&self.pattern[3]);

        // find six
        for digit in &length_six_digits {
            let digit_set = digit.chars().collect();
            if !one.is_subset(&digit_set) {
                self.pattern[6] = digit.clone();
            }
        }
        length_six_digits.remove(&self.pattern[6]);

        let six: HashSet<char> = self.pattern[6].chars().collect();
        // find zero and nine
        for digit in &length_six_digits {
            let digit_set: HashSet<char> = digit.chars().collect();
            for &x in six.difference(&digit_set) {
                if self.pattern[4].contains(x) {
                    self.pattern[0] = digit.clone();
                } else {
                    self.pattern[9] = digit.clone();
                }
            }
        }

        // find five and two
        for digit in &length_five_digits {
            let digit_set: HashSet<char> = digit.chars().collect();
            if digit_set.difference(&six).count() == 0 {
                self.pattern[5] = digit.clone();
            } else {
                self.pattern[2] = digit.clone();
            }
        }

        for (i, s) in self.pattern.clone().into_iter().enumerate() {
            self.query_map.entry(s).or_insert(i as u32);
        };
    }

    fn get_output(&self) -> u32 {
        let mut s = String::new();
        for o in &self.output {
            s.push(char::from_digit(*self.query_map.get(o).unwrap(), 10).unwrap());
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
            .map(|s| s.trim().chars().collect::<Vec<char>>())
            .collect();
        input.iter_mut().for_each(|s| s.sort());
        let input = input.iter().map(|c| String::from_iter(c)).collect();

        let mut output: Vec<Vec<char>> = split[1]
            .split_whitespace()
            .map(|s| s.trim().chars().collect())
            .collect();
        output.iter_mut().for_each(|s| s.sort());
        let output = output.iter().map(|c| String::from_iter(c)).collect();

        Ok(Self::new(input, output))
    }
}
