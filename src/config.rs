use std::str::FromStr;
use crate::error::{Error, Result};

#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Horizontal => false,
            Direction::Vertical => true,
        }
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Direction> {
        match s {
            "horizontal" => Ok(Direction::Horizontal),
            "vertical" => Ok(Direction::Vertical),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Mode {
    Red,
    Green,
    Blue,
    Alpha,
    Luma,
}

impl Mode {
    fn default_threshold(&self) -> u8 {
        match self {
            Mode::Red   => 100,
            Mode::Green => 100,
            Mode::Blue  => 100,
            Mode::Alpha => 100,
            Mode::Luma  =>  50,
        }
    }
}

impl FromStr for Mode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Mode> {
        match s {
            "red" => Ok(Mode::Red),
            "green" => Ok(Mode::Green),
            "blue" => Ok(Mode::Blue),
            "alpha" => Ok(Mode::Alpha),
            "luma" => Ok(Mode::Luma),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub direction: Direction,
    pub mode: Mode,
    pub threshold: u8,
}

impl Config {
    pub fn new(dir: &str, mode: &str, threshold: Option<&str>) -> Self {
        let mode = Mode::from_str(mode).unwrap();
        let threshold = threshold
            .and_then(|t| t.parse::<u8>().ok())
            .unwrap_or_else(|| mode.default_threshold());
        // inputs should be validated by clap
        Self {
            direction: Direction::from_str(dir).unwrap(),
            mode,
            threshold,
        }
    }
}
