use std::fmt::Debug;
use crossterm::{cursor, queue, style, style::Color};
use rand::Rng;
use std::io::{Write,self};
use super::constants::*;
use super::super::debug::debugger::log;

//               shape          color       shadow color
type Info = ([[[u16; 2]; 4]; 4], Color, Color);

#[derive(Copy, Clone,Debug)]
enum TetrominoState {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone,Debug)]
enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Copy, Clone)]
pub struct Tetromino {
    state: TetrominoState,
    block_type: TetrominoType,
    // 0,0 is the top left corner
    // x,y is the position of the top left
    pos: [i16; 2],
    last_state: TetrominoState,
    last_pos: [i16; 2],
}

impl Debug for Tetromino {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tetromino")
            .field("state", &self.state)
            .field("block_type", &self.block_type)
            .field("pos", &self.pos)
            .field("last_state", &self.last_state)
            .field("last_pos", &self.last_pos)
            .finish()
    }
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
            [[2, 0], [0, 1], [1, 1], [2, 1]],
            [[1, 0], [1, 1], [1, 2], [2, 2]],
            [[0, 1], [1, 1], [2, 1], [0, 2]],
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
            last_pos: INIT_LAST_POS,
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
        //print!("{:?}", direction);
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
    pub fn draw_itself(&self, out: &mut impl Write)-> io::Result<()> {
        self.draw_position(out, [(GAME_AREA_POSITION[0] as i16 + self.pos[0] * 2) as u16, (GAME_AREA_POSITION[1] as i16 + self.pos[1]) as u16], false)?;
        {
            //print!("{:?}", self.points());
            out.flush()?;
        }
        Ok(())
    }
    pub fn draw_position(&self, out: &mut impl Write, pos: [u16; 2], shadow: bool)-> io::Result<()> {
        let info = self.get_info();
        if shadow {
            queue!(out,style::SetForegroundColor(info.2))?;
        } else {
            queue!(out,style::SetForegroundColor(info.1))?;
        }
        let state_number = match self.state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        if shadow {
            for i in 0..4 {
                queue!(out, cursor::MoveTo(pos[0] +2* info.0[state_number][i][0], pos[1] +info.0[state_number][i][1]),
            style::Print('□'))?;
            }
        } else {
            for i in 0..4 {
                log(&format!("{:?}|{:?}\n", (pos[0] as i16 +2* info.0[state_number][i][0] as i16)as u16,( pos[1] as i16 +info.0[state_number][i][1] as i16)as u16));
                queue!(out, cursor::MoveTo((pos[0] as i16 +2* info.0[state_number][i][0] as i16)as u16,( pos[1] as i16 +info.0[state_number][i][1] as i16)as u16),
            style::Print('■'))?;
            }
        }
        queue!(out,style::SetForegroundColor(Color::Reset))?;
        out.flush()?;
        Ok(())
    }
    pub fn erase_last(&self, out: &mut impl Write)->io::Result<()> {
        let info = self.get_info();
        let state_number = match self.last_state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        for i in 0..4 {
            queue!(out, cursor::MoveTo((GAME_AREA_POSITION[0]as i16+
               self.last_pos[0]*2+
                2* info.0[state_number][i][0]as i16)as u16,
                (GAME_AREA_POSITION[1]as i16+
                self.last_pos[1]+
                info.0[state_number][i][1]as i16)as u16),
            style::Print(" "))?;
            // print!("{},{}" ,self.last_pos[0],self.last_pos[1])
        }
        out.flush()?;
        Ok(())
    }
    pub fn points(&self) -> [[i16; 2]; 4] {
        let info = self.get_info();
        let state_number = match self.state {
            TetrominoState::Up => 0,
            TetrominoState::Right => 1,
            TetrominoState::Down => 2,
            TetrominoState::Left => 3,
        };
        let mut pos_set = [[0, 0]; 4];
        for i in 0..4 {
            pos_set[i] = [
                self.pos[0] +
                    info.0[state_number][i][0] as i16,
                self.pos[1] +
                    info.0[state_number][i][1] as i16];
        }
        pos_set
    }
    pub fn color(&self) -> Color {
        self.get_info().1
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