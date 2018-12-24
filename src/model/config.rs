#[derive(Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub enum Mode {
    Red,
    Green,
    Blue,
    Alpha,
    Luma,
}

#[derive(Debug)]
pub struct Config {
    direction: Direction,
    mode: 
}