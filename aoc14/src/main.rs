use std::error::Error;
use std::io::{self, Read, Write};
use std::collections::HashMap;

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()>{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input: Vec<&str> = input.lines().collect();
    let template: Vec<u8> = input[0].bytes().collect();
    let mut rules = HashMap::new();
    for line in &input[2..] {
        if line.contains(" -> ") {
            let line: Vec<&str> = line.split(" -> ").collect();
            let key = line[0].trim().as_bytes();
            let key: (&u8, &u8) = (&key[0], &key[1]);
            let value = line[1].trim().bytes().nth(0).unwrap();
            rules.insert(key, value);
        }
    }

    writeln!(io::stdout(), "Template: {:?}", String::from_utf8(template.clone()))?;
    let mut step = 0;
    let mut counter = HashMap::new();
    let mut poly = HashMap::new();
    *counter.entry(&template[0]).or_insert(0i64) += 1;
    for pair in template.iter().zip(template[1..].iter()) {
        *counter.entry(pair.1).or_insert(0) += 1;
        *poly.entry(pair).or_insert(0) += 1;
    }

    while step < 40 {
        step += 1;
        let mut new_poly = HashMap::new();
        for (pair, times) in poly {
            let value = rules.get(&pair).unwrap();
            *new_poly.entry((pair.0, value)).or_insert(0) += times;
            *new_poly.entry((value, pair.1)).or_insert(0) += times;
            *counter.entry(value).or_insert(0) += times;
        }
        poly = new_poly;
        let most_common  = counter.iter().map(|(_, v)| v).max().unwrap();
        let least_common  = counter.iter().map(|(_, v)| v).min().unwrap();
        writeln!(io::stdout(), "After step {}, result: {}", step, most_common - least_common)?;
    }

    Ok(())
}
