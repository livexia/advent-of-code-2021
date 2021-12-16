use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Number = u64;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let transmission: String = input
        .trim()
        .chars()
        .map(|c| hex_to_bin(c))
        .collect::<Result<String>>()?;
    writeln!(io::stdout(), "There is {} bits", transmission.len())?;

    let bits: Vec<char> = transmission.clone().chars().collect();

    let mut version_sum = 0;

    let result = parse(0, &bits, &mut version_sum)?;

    writeln!(
        io::stdout(),
        "Part1: sum of the version numbers in all packets is {}",
        version_sum
    )?;
    writeln!(
        io::stdout(),
        "Part2: hexadecimal-encoded BITS transmission produces {}",
        result.1
    )?;
    Ok(())
}

fn parse(index: usize, bits: &[char], version_sum: &mut Number) -> Result<(usize, Number)> {
    let version = &bits[index..index + 3];
    *version_sum += bin_to_dec(version)?;
    let type_id = bin_to_dec(&bits[index + 3..index + 6])?;
    let mut literal_value: Vec<char> = vec![];
    if type_id == 4 {
        for start in (index + 6..).step_by(5) {
            literal_value.extend(&bits[start + 1..start + 5]);
            if bits[start] == '0' {
                let literal_value = bin_to_dec(&literal_value)?;
                return Ok((start + 5, literal_value));
            }
        }
    } else {
        let length_type_id = &bits[index + 6];
        let mut next_index = index + 7;
        let mut values = vec![];
        if length_type_id == &'0' {
            let total_length_indicated =
                next_index + 15 + bin_to_dec(&bits[next_index..next_index + 15])? as usize;
            next_index = next_index + 15;
            while next_index < total_length_indicated {
                let sub_packet = parse(next_index, &bits, version_sum)?;
                next_index = sub_packet.0;
                values.push(sub_packet.1);
            }
        } else {
            let sub_packets_number = bin_to_dec(&bits[next_index..next_index + 11])?;
            next_index = next_index + 11;
            for _ in 0..sub_packets_number {
                let sub_packet = parse(next_index, &bits, version_sum)?;
                next_index = sub_packet.0;
                values.push(sub_packet.1);
            }
        }
        let value: Number = match type_id {
            0 => values.iter().sum(),
            1 => values.iter().fold(1, |acc, v| acc * v),
            2 => *values.iter().min().unwrap(),
            3 => *values.iter().max().unwrap(),
            5 => {
                let a = values[0];
                let b = values[1];
                if a > b {
                    1
                } else {
                    0
                }
            }
            6 => {
                let a = values[0];
                let b = values[1];
                if a < b {
                    1
                } else {
                    0
                }
            }
            7 => {
                let a = values[0];
                let b = values[1];
                if a == b {
                    1
                } else {
                    0
                }
            }
            _ => return err!("Wrong operator: {}", type_id),
        };
        return Ok((next_index, value));
    }
    return err!("Parse {:?} error, start at: {}", bits, index);
}

fn bin_to_dec(input: &[char]) -> Result<Number> {
    let s: String = input.iter().collect();
    if let Ok(num) = Number::from_str_radix(&s, 2) {
        Ok(num)
    } else {
        err!("Wrong binary number: {}, lenght: {}", s, s.len())
    }
}

fn hex_to_bin(c: char) -> Result<&'static str> {
    match c {
        '0' => Ok("0000"),
        '1' => Ok("0001"),
        '2' => Ok("0010"),
        '3' => Ok("0011"),
        '4' => Ok("0100"),
        '5' => Ok("0101"),
        '6' => Ok("0110"),
        '7' => Ok("0111"),
        '8' => Ok("1000"),
        '9' => Ok("1001"),
        'A' => Ok("1010"),
        'B' => Ok("1011"),
        'C' => Ok("1100"),
        'D' => Ok("1101"),
        'E' => Ok("1110"),
        'F' => Ok("1111"),
        _ => err!("Wrong hexadecimal: {}", c),
    }
}
