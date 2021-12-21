use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};

macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let starting_pos: Vec<u32> = input
        .lines()
        .map(|s| s.split(": ").last().unwrap().parse::<u32>().unwrap())
        .collect();
    writeln!(io::stdout(), "starting position is: {:?}", starting_pos)?;

    let mut deterministic_die = (1..=100).cycle();
    let mut player1 = Player::new(0, starting_pos[0], 1000);
    let mut player2 = Player::new(1, starting_pos[1], 1000);
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

    let player1 = Player::new(0, starting_pos[0], 21);
    let player2 = Player::new(1, starting_pos[1], 21);
    let mut games = vec![Game::new(player1, player2)];
    let mut counter = vec![0u64, 0u64];

    let mut dirac_die: HashMap<u32, u64> = HashMap::new();
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                *dirac_die.entry(i + j + k).or_insert(0) += 1;
            }
        }
    }

    while let Some(game) = games.pop() {
        for game in game.roll_dirac_die(&dirac_die) {
            if let Some(key) = game.win() {
                counter[key] += game.players[1].times;
            } else {
                games.push(game)
            }
        }
    }
    writeln!(
        io::stdout(),
        "Part2: the player that wins in more universes totaly win {:?} in universes",
        counter.iter().max().unwrap()
    )?;

    Ok(())
}

#[derive(Debug)]
struct Game {
    players: [Player; 2],
}

impl Game {
    fn new(player1: Player, player2: Player) -> Self {
        Self {
            players: [player1, player2],
        }
    }

    fn roll_dirac_die(&self, die: &HashMap<u32, u64>) -> Vec<Self> {
        self.players[0].roll_dirac_die(self.players[1], die)
    }

    fn win(&self) -> Option<usize> {
        if self.players[1].win() {
            Some(self.players[1].index)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Player {
    index: usize,
    pos: u32,
    score: u32,
    max: u32,
    times: u64,
}

impl Player {
    fn new(index: usize, pos: u32, max: u32) -> Self {
        Self {
            index,
            pos,
            score: 0,
            max,
            times: 1,
        }
    }

    fn roll_deterministic_die<D>(&mut self, die: &mut D)
    where
        D: Iterator<Item = u32>,
    {
        self.pos += die.next().unwrap();
        self.pos += die.next().unwrap();
        self.pos += die.next().unwrap();
        while self.pos > 10 {
            self.pos -= 10
        }
        self.score += self.pos;
    }

    fn clone_with_offset(&mut self, offset: u32, times: u64) -> Self {
        let mut pos = self.pos + offset;
        if pos > 10 {
            pos -= 10
        }
        Self {
            pos,
            times: self.times * times,
            score: self.score + pos,
            ..*self
        }
    }

    fn appear_times(mut self, times: u64) -> Self {
        self.times *= times;
        self
    }

    fn roll_dirac_die(mut self, other: Player, die: &HashMap<u32, u64>) -> Vec<Game> {
        die.iter()
            .map(|(&i, &t)| Game::new(other.appear_times(t), self.clone_with_offset(i, t)))
            .collect()
    }

    fn win(&self) -> bool {
        self.score >= self.max
    }
}
