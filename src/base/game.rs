use std::io::Write;
use crate::base::tetromino::Tetromino;
use super::frame;
use std::collections::LinkedList;
use std::thread::sleep;
use super::constants::*;
use crossterm::{queue, style::Print, cursor, execute, style, terminal, event::{self, KeyCode}};
use crossterm::event::read;
use crossterm::style::{Color, style};
use crate::base::constants::{Direction, GAME_AREA_SIZE};

type PointState = Option<Color>;

struct Matrix {
    data: Vec<//20,num of rows
        Vec<//10,num of cols
            PointState>>,
    width: u16,
    height: u16,
}

enum GameMode {
    Normal
}

struct Game {
    mode: GameMode,
    difficulty: u8,
    queue: LinkedList<Tetromino>,
    //todo: Atomic
    score: u64,
    current: Tetromino,
    grid: Matrix,
}

impl Game {
    fn new(out: &mut impl Write, mode: GameMode, difficulty: u8) -> Game {
        let mut n = Self {
            mode,
            difficulty,
            queue: Default::default(),
            score: 0,
            current: Tetromino::new(),
            grid: Matrix {
                data: vec![vec![None; GAME_AREA_SIZE[0] as usize]; GAME_AREA_SIZE[1] as usize],
                width: GAME_AREA_SIZE[0],
                height: GAME_AREA_SIZE[1],
            },
        };
        n
    }
    fn game_loop(&mut self, out: &mut impl Write) {
        self.queue.push_back(Tetromino::new());
        //one thread to refresh info
        terminal::enable_raw_mode();
        'new_tetromino: loop {
            //TODO: make this async
            self.queue.push_back(Tetromino::new());
            self.current = self.queue.pop_front().expect("can not get a tetromino");
            //draw_next_tip first for using the current.last_pos
            self.draw_next_tip(out);
            self.draw_score(out);
            self.set_new_tetromino(out);
            out.flush();
            'event: loop {
                match read().expect("can not read event") {
                    event::Event::Key(key_event) => {
                        match key_event.code {
                            KeyCode::Esc => {
                                terminal::disable_raw_mode();
                                break 'new_tetromino;
                            }
                            KeyCode::Left | KeyCode::Char('a' | 'A') => {
                                if self.move_tetromino_left(out) == false
                                { continue 'event; }
                            }
                            KeyCode::Right | KeyCode::Char('d' | 'D') => {
                                if self.move_tetromino_right(out) == false
                                { continue 'event; }
                            }
                            KeyCode::Up | KeyCode::Char('w' | 'W') => {
                                if self.rotate_tetromino(out) == false
                                { continue 'event; }
                            }
                            KeyCode::Down | KeyCode::Char('s' | 'S') => {
                                if self.move_tetromino_down(out) == false
                                { break 'event; }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }


            if (false) {
                terminal::disable_raw_mode();
                break;
            }
        }
    }
}

impl Game {
    fn set_new_tetromino(&mut self, out: &mut impl Write) {
        self.current.init_to_game();
        self.current.draw_itself(out);
    }
    //true: can move. false: fuse
    fn move_tetromino_down(&mut self, out: &mut impl Write) -> bool {
        let points = self.current.points();

        //have been drawn to canvas in this loop if fused
        for point in points.iter() {
            if point[1] as u16 == self.grid.height - 1 {
                self.fuse_tetromino(out);
                return false;
            }
            if self.grid.data[point[1] as usize + 1][point[0] as usize].is_some() {
                self.fuse_tetromino(out);
                return false;
            }
        }
        self.current.shift(Direction::Down);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
        true
    }
    //true: can move; false: can't move
    fn move_tetromino_left(&mut self, out: &mut impl Write) -> bool {
        let points = self.current.points();
        for point in points.iter() {
            if point[0] == 0 || self.grid.data[point[1] as usize][point[0] as usize - 1].is_some() {
                return false;
            }
        }
        self.current.shift(Direction::Left);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
        true
    }
    fn move_tetromino_right(&mut self, out: &mut impl Write) -> bool {
        let points = self.current.points();
        for point in points.iter() {
            if point[0] as u16 == self.grid.width  - 1 || self.grid.data[point[1] as usize][point[0] as usize + 1].is_some() {
                return false;
            }
        }
        self.current.shift(Direction::Right);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
        true
    }
    //TODO: kick wall, now can't rotate
    fn rotate_tetromino(&mut self, out: &mut impl Write) -> bool {
        self.current.rotate();
        let points = self.current.points();
        for point in points.iter() {
            if point[0] as u16 >= self.grid.width || point[1] as u16 >= self.grid.height || self.grid.data[point[1] as usize][point[0] as usize].is_some() {
                self.current.reverse_back();
                return false;
            }
        }
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
        true
    }
}

impl Game {
    fn draw_next_tip(&self, out: &mut impl Write) {
        let &next = &self.queue.front().unwrap();
        self.current.erase_last(out);
        next.draw_position(out, super::constants::NEXT_TIP_TETROMINO);
    }
    fn draw_score(&self, out: &mut impl Write) {
        queue!(out,cursor::MoveTo(31,9),style::SetForegroundColor(Color::DarkYellow),Print(self.score),style::SetForegroundColor(Color::Reset));
    }
    fn fuse_tetromino(&mut self, out: &mut impl Write) {
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
        let points = self.current.points();
        for point in points.iter() {
            // print!("{:?}", point);
            self.grid.data[point[1] as usize][point[0] as usize] = Some(self.current.color());
        }
        // print!("fuse fin");
    }
}


pub fn start(out: &mut impl Write) {
    let mut game_instance = Game::new(out, GameMode::Normal, 0);
    frame::draw_frame(out);
    game_instance.game_loop(out);
}

