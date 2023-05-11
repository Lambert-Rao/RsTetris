use crossterm::{cursor, queue, style, style::Color};
use rand::Rng;
use std::io::Write;
use crossterm::style::Print;
use super::constants::*;
use crate::base::frame::draw_frame;

type Info = ([[[u16; 2]; 4]; 4], Color, Color);

#[derive(Copy, Clone)]
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
    pos: [u16; 2],
    last_state: TetrominoState,
    last_pos: [u16; 2],
}

//const info
impl TetrominoType {
    //SRS
    const I_INFO: Info = (
        [
            [[0, 1], [1, 1], [2, 1], [3, 1]],
            [[2, 0], [2, 1], [2, 2], [2, 3]],
            [[0, 2], [1, 2], [2, 2], [3, 2]],
            [[1, 0], [1, 1], [1, 2], [1, 3]]
        ],
        Color::Rgb { r: 0, g: 240, b: 240 },
        Color::Rgb { r: 0, g: 160, b: 160 }
    );
    const O_INFO: Info = (
        [
            [[1, 0], [2, 0], [1, 1], [2, 1]],
            [[1, 0], [2, 0], [1, 1], [2, 1]],
            [[1, 0], [2, 0], [1, 1], [2, 1]],
            [[1, 0], [2, 0], [1, 1], [2, 1]]
        ],
        Color::Rgb { r: 240, g: 240, b: 0 },
        Color::Rgb { r: 160, g: 160, b: 0 }
    );
    const J_INFO: Info = (
        [
            [[0, 0], [0, 1], [1, 1], [2, 1]],
            [[1, 0], [2, 0], [1, 1], [1, 2]],
            [[0, 1], [1, 1], [2, 1], [2, 2]],
            [[1, 0], [1, 1], [0, 2], [1, 2]],
        ],
        Color::Rgb { r: 0, g: 0, b: 240 },
        Color::Rgb { r: 0, g: 0, b: 160 }
    );
    const L_INFO: Info = (
        [
            [[0, 1], [1, 1], [2, 1], [2, 0]],
            [[1, 0], [1, 1], [1, 2], [2, 2]],
            [[0, 2], [0, 1], [1, 1], [2, 1]],
            [[0, 0], [1, 0], [1, 1], [1, 2]],
        ],
        Color::Rgb { r: 240, g: 160, b: 0 },
        Color::Rgb { r: 160, g: 120, b: 0 }
    );
    const S_INFO: Info = (
        [
            [[1, 0], [2, 0], [0, 1], [1, 1]],
            [[1, 0], [1, 1], [2, 1], [2, 2]],
            [[1, 1], [2, 1], [0, 2], [1, 2]],
            [[0, 0], [0, 1], [1, 1], [1, 2]]
        ],
        Color::Rgb { r: 0, g: 240, b: 0 },
        Color::Rgb { r: 0, g: 160, b: 0 }
    );
    const T_INFO: Info = (
        [
            [[1, 0], [0, 1], [1, 1], [2, 1]],
            [[1, 0], [1, 1], [2, 1], [1, 2]],
            [[0, 1], [1, 1], [2, 1], [1, 2]],
            [[1, 0], [0, 1], [1, 1], [1, 2]],
        ],
        Color::Rgb { r: 60, g: 0, b: 240 },
        Color::Rgb { r: 120, g: 0, b: 160 }
    );
    const Z_INFO: Info = (
        [
            [[0, 0], [1, 0], [1, 1], [2, 1]],
            [[2, 0], [1, 1], [2, 1], [1, 2]],
            [[0, 1], [1, 1], [1, 2], [2, 2]],
            [[1, 0], [0, 1], [1, 1], [0, 2]],
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
            last_state: TetrominoState::Up,
            pos: [0, 0],
            last_pos: NEXT_TIP_TETROMINO,
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
    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Left => self.pos[0] -= 1,
            Direction::Right => self.pos[0] += 1,
            Direction::Down => self.pos[1] += 1,
            _ => {}
        }
    }
    pub fn set_last(&mut self) {
        self.last_state = self.state;
        self.last_pos = self.pos;
    }
    pub fn init_to_game(&mut self) {
        self.pos = [0, 0];
        self.last_pos = self.pos.clone();
    }
    pub fn reverse_back(&mut self) {
        self.pos = self.last_pos;
        self.state = self.last_state;
    }
    //this draw covers 8*4 grid
    pub fn draw_itself(&self, out: &mut impl Write) {
        self.draw_position(out, [GAME_AREA_POSITION[0] + self.pos[0] * 2, GAME_AREA_POSITION[1] + self.pos[1]]);
    }
    pub fn draw_position(&self, out: &mut impl Write, pos: [u16; 2]) {
        let info = self.get_info();
        queue!(out,style::SetForegroundColor(info.1));
        let state_number = match self.state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        for i in 0..4 {
            queue!(out, cursor::MoveTo(pos[0] +2* info.0[state_number][i][0], pos[1] +info.0[state_number][i][1]),
            style::Print('■'));
        }
        queue!(out,style::SetForegroundColor(Color::Reset));
        out.flush();
    }
    pub fn erase_last(&self, out: &mut impl Write) {
        let info = self.get_info();
        let state_number = match self.last_state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        for i in 0..4 {
            queue!(out, cursor::MoveTo(GAME_AREA_POSITION[0]+
                self.last_pos[0]*2+
                2* info.0[state_number][i][0],
                GAME_AREA_POSITION[1]+
                self.last_pos[1]+
                info.0[state_number][i][1]),
            style::Print(" "));
            // print!("{},{}" ,self.last_pos[0],self.last_pos[1])
        }
        out.flush();
    }
}

impl Tetromino {
    fn get_info(&self) -> Info {
        match self.block_type {
            TetrominoType::I => TetrominoType::I_INFO,
            TetrominoType::O => TetrominoType::O_INFO,
            TetrominoType::J => TetrominoType::J_INFO,
            TetrominoType::L => TetrominoType::L_INFO,
            TetrominoType::S => TetrominoType::S_INFO,
            TetrominoType::T => TetrominoType::T_INFO,
            TetrominoType::Z => TetrominoType::Z_INFO,
        }
    }
}