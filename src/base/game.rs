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
            loop{match read().expect("can not read event") {
                event::Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc => {
                            terminal::disable_raw_mode();
                            break 'new_tetromino
                        }
                        KeyCode::Left|KeyCode::Char('a'|'A') => {
                            self.move_tetromino_left(out);
                        }
                        KeyCode::Right|KeyCode::Char('d'|'D') => {
                            self.move_tetromino_right(out);
                        }
                        KeyCode::Up|KeyCode::Char('w'|'W') => {
                            self.rotate_tetromino(out);
                        }
                        KeyCode::Down|KeyCode::Char('s'|'S') => {
                            self.move_tetromino_down(out);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }}


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
    fn move_tetromino_down(&mut self, out: &mut impl Write) {
        let points = self.current.points();

        //have been drawn to canvas in this loop if fused
        for point in points.iter() {
            print!("{:?}", point);
            if point[1] == self.grid.height -1 {
                print!("fuse");
                self.fuse_tetromino(out);
                return;
            }
            if self.grid.data[point[1] as usize + 1][point[0] as usize].is_some() {
                self.fuse_tetromino(out);
                return;
            }
        }
        self.current.shift(Direction::Down);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
    }
    fn move_tetromino_left(&mut self, out: &mut impl Write) {
        self.current.shift(Direction::Left);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
    }
    fn move_tetromino_right(&mut self, out: &mut impl Write) {
        self.current.shift(Direction::Right);
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
    }
    fn rotate_tetromino(&mut self, out: &mut impl Write) {
        self.current.rotate();
        self.current.erase_last(out);
        self.current.set_last();
        self.current.draw_itself(out);
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
            print!("{:?}", point);
            self.grid.data[point[1] as usize][point[0] as usize] = Some(self.current.color());
        }
        print!("fuse fin");
    }
}


pub fn start(out: &mut impl Write) {
    let mut game_instance = Game::new(out, GameMode::Normal, 0);
    frame::draw_frame(out);
    game_instance.game_loop(out);
}

