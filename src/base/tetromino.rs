use crossterm::{cursor, queue, style, style::Color};
use rand::Rng;
use std::io::Write;
use crossterm::style::Print;

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
}

//const info
impl Tetromino {
    const I: Info = (
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
        Color::Rgb { r: 240, g: 240, b: 0 },
        Color::Rgb { r: 160, g: 160, b: 0 }
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
        Color::Rgb { r: 0, g: 0, b: 240 },
        Color::Rgb { r: 0, g: 0, b: 160 }
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
        Color::Rgb { r: 240, g: 160, b: 0 },
        Color::Rgb { r: 160, g: 120, b: 0 }
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
        Color::Rgb { r: 0, g: 240, b: 0 },
        Color::Rgb { r: 0, g: 160, b: 0 }
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
        Color::Rgb { r: 60, g: 0, b: 240 },
        Color::Rgb { r: 120, g: 0, b: 160 }
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
        Color::Rgb { r: 240, g: 0, b: 0 },
        Color::Rgb { r: 160, g: 0, b: 0 }
    );
}

impl Tetromino {
    pub fn new() -> Tetromino {
        let t_type = match rand::thread_rng().gen_range(0..7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::J,
            3 => TetrominoType::L,
            4 => TetrominoType::S,
            5 => TetrominoType::T,
            6 => TetrominoType::Z,
            _ => unreachable!()
        };
        let new_tetromino = Tetromino {
            block_type: t_type,
            state: TetrominoState::Up,
        };
        new_tetromino
    }
    pub fn rotate(&mut self) {
        self.state = match self.state {
            TetrominoState::Up => TetrominoState::Right,
            TetrominoState::Right => TetrominoState::Down,
            TetrominoState::Down => TetrominoState::Left,
            TetrominoState::Left => TetrominoState::Up,
        }
    }
    //this draw takes 8*4 grid
    pub fn draw(&self, out: &mut impl Write, pos: [u16; 2]) {
        let info = match self.block_type {
            TetrominoType::I => Tetromino::I,
            TetrominoType::O => Tetromino::O,
            TetrominoType::J => Tetromino::J,
            TetrominoType::L => Tetromino::L,
            TetrominoType::S => Tetromino::S,
            TetrominoType::T => Tetromino::T,
            TetrominoType::Z => Tetromino::Z,
        };
        queue!(out,style::SetForegroundColor(info.1));
        let state_number = match self.state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        for i in 0..4 {
            for j in 0..4 {
                if info.0[state_number][i][j] {
                    queue!(out, cursor::MoveTo(pos[0] + (j*2) as u16, pos[1] + i as u16),Print("■ "));
                }
            }
        }
        queue!(out,style::SetForegroundColor(Color::Reset));
        out.flush();
    }
}