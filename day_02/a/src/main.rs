use std::fs::read_to_string;

//opponent
//A for Rock, B for Paper, and C for Scissors
const O_ROCK: &str = "A";
const O_PAPER: &str = "B";
const O_SCISSORS: &str = "C";

//player
//X for Rock, Y for Paper, and Z for Scissors
const P_ROCK: &str = "X";
const P_PAPER: &str = "Y";
const P_SCISSORS: &str = "Z";

//res
//X means lose, Y means draw, and Z means win
const R_LOSE: &str = "X";
const R_DRAW: &str = "Y";
const R_WIN: &str = "Z";

fn main() {
    let str = read_to_string("../input.txt").unwrap();
    // let str = "A Y\nB X\nC Z";

    let strat = str.split("\n").map_while(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() == 2 {
            Some((parts[0], parts[1]))
        } else {
            None
        }
    });

    let scores = strat.map(|(o, p)| {
        let o_play = Play::from_o(o);
        let p_play = Play::from_p(p);
        let res = p_play.against(&o_play);
        (res.to_points() + p_play.to_points()) as u64
    });

    let total_score = scores.reduce(|a, b| a + b).unwrap();

    println!("score: {}", total_score);
}

#[derive(Debug, PartialEq)]
enum PlayResult {
    Win,
    Lose,
    Draw,
}
impl PlayResult {
    fn to_points(&self) -> u8 {
        //(0 if you lost, 3 if the round was a draw, and 6 if you won)
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
    fn from_str(str: &str) -> Self {
        match str {
            R_LOSE => Self::Lose,
            R_DRAW => Self::Draw,
            R_WIN => Self::Win,
            _ => panic!("invalid res char: {}", str),
        }
    }
}

#[derive(Clone)]
enum Play {
    Rock,
    Paper,
    Scissors,
}
impl Play {
    fn against(&self, o_play: &Self) -> PlayResult {
        match self {
            Self::Rock => match o_play {
                Self::Rock => PlayResult::Draw,
                Self::Paper => PlayResult::Lose,
                Self::Scissors => PlayResult::Win,
            },
            Self::Paper => match o_play {
                Self::Rock => PlayResult::Win,
                Self::Paper => PlayResult::Draw,
                Self::Scissors => PlayResult::Lose,
            },
            Self::Scissors => match o_play {
                Self::Rock => PlayResult::Lose,
                Self::Paper => PlayResult::Win,
                Self::Scissors => PlayResult::Draw,
            },
        }
    }
    fn to_points(&self) -> u8 {
        //(1 for Rock, 2 for Paper, and 3 for Scissors)
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
    fn from_o_res(o_play: Play, res: PlayResult) -> Self {
        [Self::Paper, Self::Rock, Self::Scissors]
            .iter()
            .find(|p| p.against(&o_play) == res)
            .unwrap()
            .to_owned()
    }
    fn from_o(char: &str) -> Self {
        match char {
            O_ROCK => Self::Rock,
            O_PAPER => Self::Paper,
            O_SCISSORS => Self::Scissors,
            _ => panic!("invalid char: {}", char),
        }
    }
    fn from_p(char: &str) -> Self {
        match char {
            P_ROCK => Self::Rock,
            P_PAPER => Self::Paper,
            P_SCISSORS => Self::Scissors,
            _ => panic!("invalid char: {}", char),
        }
    }
}
