use crossterm::{cursor, queue, style, style::Color};
use rand::Rng;
use crate::base::tetromino::TetrominoState::Up;

type Matrix = [[bool; 4]; 4];
type Info = ([Matrix; 4], Color, Color);


enum TetrominoState {
    Up,
    Right,
    Down,
    Left,
}

enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub struct Tetromino {
    state: TetrominoState,
    block_type: TetrominoType,
    // 0,0 is the top left corner
    // x,y is the position of the top left
    x: usize,
    y: usize,
}

//const info
impl Tetromino {
    pub const I: Info = (
        [
            [
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
                [false, false, true, false],
            ],
            [
                [false, false, false, false],
                [false, false, false, false],
                [true, true, true, true],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
            ],
        ],
        Color::Rgb { r: 0, g: 240, b: 240 },
        Color::Rgb { r: 0, g: 160, b: 160 }
    );
    const O: Info = (
        [
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb { r:240,g: 240,b: 0 },
        Color::Rgb { r:160, g: 160, b:0 }
    );
    const J: Info = (
        [
            [
                [false, false, false, false],
                [true, false, false, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, true, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb {r: 0,g:  0, b:240 },
        Color::Rgb { r:0,g:  0,b:160 }
    );
    const L: Info = (
        [
            [
                [false, false, false, false],
                [false, false, true, false],
                [true, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, true, false],
                [true, false, false, false],
                [false, false, false, false],
            ],
            [
                [true, true, false, false],
                [false, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb {r: 240, g: 160,b: 0 },
        Color::Rgb { r:160,g:  120, b:0 }
    );
    const S: Info = (
        [
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [false, true, true, false],
                [true, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, false, true, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb { r:0, g: 240,b: 0 },
        Color::Rgb { r:0, g: 160, b:0 }
    );
    const T: Info = (
        [
            [
                [false, false, false, false],
                [true, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, false, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [true, true, true, false],
                [false, false, false, false],
                [false, false, false, false],
            ],
            [
                [false, true, false, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb { r:60,g: 0, b:240 },
        Color::Rgb { r:120,g:  0, b:160 }
    );
    const Z: Info = (
        [
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
            [
                [false, false, false, false],
                [true, true, false, false],
                [false, true, true, false],
                [false, false, false, false],
            ],
            [
                [false, false, true, false],
                [false, true, true, false],
                [false, true, false, false],
                [false, false, false, false],
            ],
        ],
        Color::Rgb {r: 240, g: 0, b:0 },
        Color::Rgb {r: 160,g:  0, b:0 }
    );
}

impl Tetromino {
    fn new() -> ! { panic!("Tetromino must be initialized with a type") }
}