use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::process::exit;
use std::time::{Duration, Instant};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

type Cache = HashMap<((usize, usize), (usize, usize)), (usize, usize)>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let pos: Vec<u32> = input
        .lines()
        .map(|s| s.split(": ").last().unwrap().parse::<u32>().unwrap() - 1)
        .collect();
    writeln!(io::stdout(), "starting position is: {:?}", pos)?;

    let start = Instant::now();
    part1(&pos)?;
    writeln!(io::stdout(), "Part 1: took {:?} to cumpute", Instant::now() - start)?;
    let start = Instant::now();
    part2(&pos)?;
    writeln!(io::stdout(), "Part 2: took {:?} to cumpute", Instant::now() - start)?;
    let start = Instant::now();
    part2_with_cache(&pos)?;
    writeln!(io::stdout(), "Part 2 with cache: took {:?} to cumpute", Instant::now() - start)?;

    Ok(())
}

fn part1(pos: &[u32]) -> Result<()> {
    let mut deterministic_die = (1..=100).cycle();
    let mut player1 = Player::new(pos[0], 1000);
    let mut player2 = Player::new(pos[1], 1000);
    let mut players = vec![&mut player1, &mut player2];

    let mut times = 0;
    'outer: loop {
        for player in &mut players {
            times += 3;
            player.roll_deterministic_die(&mut deterministic_die);
            if player.win() {
                break 'outer;
            }
        }
    }
    writeln!(
        io::stdout(),
        "Part1: multiply the score of the losing player by the number \
        of times the die was rolled during the game is {}",
        times * player1.score.min(player2.score)
    )?;
    Ok(())
}

fn part2(pos: &[u32]) -> Result<()> {
    let mut games = vec![Game::new((pos[0], pos[1]), (0, 0), 0, 1)];
    let mut counter = vec![0u64, 0u64];

    let mut dirac_die: HashMap<u32, u64> = HashMap::new();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                *dirac_die.entry(i + j + k).or_insert(0) += 1;
            }
        }
    }
    let dirac_die: Vec<(u32, u64)> = dirac_die.iter()
        .map(|(&k, &v)| (k, v))
        .collect();

    while let Some(game) = games.pop() {
        for (i, t) in &dirac_die {
            let new_game = game.next_roll(i, t * game.times);
            if let Some(key) = new_game.win() {
                counter[key] += new_game.times;
            } else {
                games.push(new_game)
            }
        }
    }

    writeln!(
        io::stdout(),
        "Part2: the player that wins in more universes totaly win {:?} in universes",
        counter.iter().max().unwrap()
    )?;

    assert_eq!(counter.iter().max().unwrap(), &92399285032143);
    Ok(())
}

fn part2_with_cache(pos: &[u32]) -> Result<()> {
    let mut cache = HashMap::new();
    let (s1, s2) = quantum_game(&mut cache, (pos[0] as usize, pos[1] as usize), (0, 0));

    writeln!(
        io::stdout(),
        "Part2: the player that wins in more universes totaly win {:?} in universes",
        s1.max(s2)
    )?;

    assert_eq!(s1.max(s2), 92399285032143);
    Ok(())
}

fn quantum_game(cache: &mut Cache, pos: (usize, usize), score: (usize, usize)) -> (usize, usize) {
    if score.1 >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(pos, score)) {
        return score;
    }
    let mut new_score = (0, 0);
    for (offset, times) in [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)] {
        let pos = (pos.1, (pos.0 + offset) % 10);
        let score = (score.1, score.0 + pos.1 + 1);
        let (s1, s2) = quantum_game(cache, pos, score);
        new_score = (new_score.0 + s2 * times, new_score.1 + s1 * times);
    }
    cache.insert((pos, score), new_score);
    new_score
}

#[derive(Debug)]
struct Game {
    pos: (u32, u32),
    score: (u32, u32),
    cur: usize,
    times: u64,
}

impl Game {
    fn new(pos: (u32, u32), score: (u32, u32), cur: usize, times: u64) -> Self {
        Self {
            pos,
            score,
            cur,
            times,
        }
    }


    fn next_roll(&self, offset: &u32, times: u64) -> Self {
        let pos = (self.pos.1, (self.pos.0 + offset) % 10);
        let score = (self.score.1, self.score.0 + pos.1 + 1);
        Self {
            pos,
            score,
            cur: 1 - self.cur,
            times,
        }
    }

    fn win(&self) -> Option<usize> {
        if self.score.1 >= 21 {
            Some(1 - self.cur)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Player {
    pos: u32,
    score: u32,
    max: u32,
}

impl Player {
    fn new(pos: u32, max: u32) -> Self {
        Self { pos, score: 0, max }
    }

    fn roll_deterministic_die<D>(&mut self, die: &mut D)
    where
        D: Iterator<Item = u32>,
    {
        self.pos += die.next().unwrap();
        self.pos += die.next().unwrap();
        self.pos += die.next().unwrap();
        self.pos %= 10;
        self.score += self.pos + 1;
    }

    fn win(&self) -> bool {
        self.score >= self.max
    }
}
