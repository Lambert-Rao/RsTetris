
use crossterm::{cursor, queue, style, style::Color};

mod base;

type PointState = Option<Color>;
struct Matrix{
    data:Vec<Vec<PointState>>,
    width: usize,
    height: usize
}


fn main() {
    println!("Hello, world!");
}