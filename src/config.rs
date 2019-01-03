use std::str::FromStr;
use crate::error::{Error, Result};

/// A direction of sorting
#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    /// Returns true if image is sorted vertically
    pub fn is_vertical(&self) -> bool {
        match self {
            Direction::Up
            | Direction::Down => true,
            Direction::Right
            | Direction::Left => false,
        }
    }

    /// Returns true if the direction requires a reversed sort
    pub fn is_reverse(&self) -> bool {
        match self {
            Direction::Left
            | Direction::Up => true,
            Direction::Right
            | Direction::Down => false,
        }
    }
}

impl FromStr for Direction {
    type Err = Error;
    
    fn from_str(s: &str) -> Result<Direction> {
        match s {
            "up" => Ok(Direction::Up),
            "right" => Ok(Direction::Right),
            "down" => Ok(Direction::Down),
            "left" => Ok(Direction::Left),
            _ => unreachable!(),
        }
    }
}

/// A mode to sort on
#[derive(Debug)]
pub enum Mode {
    Red,
    Green,
    Blue,
    Alpha,
    Luma,
}

impl Mode {
    /// Gets the default threshold value for a mode
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

/// Configuration options for pixel sorting
#[derive(Debug)]
pub struct Config<'a> {
    pub direction: Direction,
    pub mode: Mode,
    pub threshold: u8,
    pub bound: &'a str
}

impl<'a> Config<'a> {
    /// Creates a new config from string values
    pub fn new(dir: &str, mode: &str, threshold: Option<&str>, bound: &'a str) -> Self {
        let mode = Mode::from_str(mode).unwrap();
        let threshold = threshold
            .and_then(|t| t.parse::<u8>().ok())
            .unwrap_or_else(|| mode.default_threshold());
        // inputs should be validated by clap
        Self {
            direction: Direction::from_str(dir).unwrap(),
            mode,
            threshold,
            bound,
        }
    }
}
