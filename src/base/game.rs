use std::io::Write;
use crate::base::tetromino::Tetromino;
use super::frame;
use std::collections::LinkedList;
use std::io;
use super::constants::*;
use crossterm::{queue, style::Print, cursor, style, terminal, event::{self, KeyCode}};
use crossterm::event::read;
use crossterm::style::{Color};
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
    //In a graphic terminal?
    display: bool,
    current: Tetromino,
    shadow: Tetromino,
    grid: Matrix,
}

//Game control
impl Game {
    fn new(mode: GameMode, difficulty: u8) -> Game {
        let n = Self {
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
            shadow: Tetromino::new(),
            display: match std::env::var("DISPLAY") {
                Ok(_) => true,
                Err(_) => false,
            },
        };
        n
    }
    fn game_loop(&mut self, out: &mut impl Write) -> io::Result<()> {
        self.queue.push_back(Tetromino::new());
        //one thread to refresh info
        terminal::enable_raw_mode()?;
        'new_tetromino: loop {
            //TODO: make this async
            self.queue.push_back(Tetromino::new());
            self.current = self.queue.pop_front().expect("can not get a tetromino");
            //draw_next_tip first for using the current.last_pos
            self.draw_next_tip(out)?;
            self.draw_score(out)?;
            self.set_new_tetromino(out)?;
            out.flush()?;
            'event: loop {
                 match read().expect("can not read event") {
                    event::Event::Key(key_event) => {
                        match key_event {
                            event::KeyEvent {
                                code: KeyCode::Esc,
                                ..
                            } => {
                                terminal::disable_raw_mode()?;
                                break 'new_tetromino;
                            }
                            event::KeyEvent {
                                code: KeyCode::Left | KeyCode::Char('a' | 'A' | 'h' | 'H'),
                                ..
                            } => {
                                if self.move_tetromino_left(out).unwrap() == false
                                { continue 'event; }
                            }
                            event::KeyEvent {
                                code: KeyCode::Right | KeyCode::Char('d' | 'D' | 'l' | 'L'),
                                ..
                            } => {
                                if self.move_tetromino_right(out).unwrap() == false
                                { continue 'event; }
                            }
                            event::KeyEvent {
                                code: KeyCode::Up | KeyCode::Char('w' | 'W' | 'K' | 'k'),
                                ..
                            } => {
                                if self.rotate_tetromino(out).unwrap() == false
                                { continue 'event; }
                            }
                            event::KeyEvent {
                                code: KeyCode::Down | KeyCode::Char('s' | 'S' | 'j' | 'J'),
                                modifiers: event::KeyModifiers::NONE,
                                ..
                            } => {
                                if self.move_tetromino_down(out).unwrap() == false
                                { break 'event; }
                            }
                            event::KeyEvent {
                                code: KeyCode::Down | KeyCode::Char('s' | 'S' | 'j' | 'J'),
                                modifiers: event::KeyModifiers::SHIFT,
                                ..
                            } => {
                                if self.move_tetromino_down(out).unwrap() == false
                                { break 'event; }
                            }

                            _ => {}
                        }
                    }
                    _ => {}
                }
                // after a key event or a time limit,TODO:time limit
                // self.update_shadow(out); TODO:shadow
            }

            //
            // if (false) {
            //     terminal::disable_raw_mode()?;
            //     break;
            // }
        }
        Ok(())
    }
}

//tetromino funcs
impl Game {
    fn set_new_tetromino(&mut self, out: &mut impl Write) -> io::Result<()> {
        self.current.init_to_game();
        self.current.draw_itself(out)?;
        Ok(())
    }
    //true: can move. false: fuse
    fn move_tetromino_down(&mut self, out: &mut impl Write) -> Result<bool, io::Error> {
        let points = self.current.points();

        //have been drawn to canvas in this loop if fused
        for point in points.iter() {
            if point[1] as u16 == self.grid.height - 1 {
                self.fuse_tetromino(out)?;
                return Ok(false);
            }
            if self.grid.data[point[1] as usize + 1][point[0] as usize].is_some() {
                self.fuse_tetromino(out)?;
                return Ok(false);
            }
        }
        self.current.shift(Direction::Down);
        //print!("{:?}", self.current.points());
        //erase first, then set
        self.current.erase_last(out)?;
        self.current.set_last();
        self.current.draw_itself(out)?;
        Ok(true)
    }
    //true: can move; false: can't move
    fn move_tetromino_left(&mut self, out: &mut impl Write) -> Result<bool, io::Error> {
        let points = self.current.points();
        //print!("{:?}", points);

        for point in points.iter() {
            if point[0] == 0 || self .grid.data[point[1] as usize][point[0] as usize - 1].is_some() {
                return Ok(false);
            }
        }
        self.current.shift(Direction::Left);
        self.current.erase_last(out)?;
        self.current.set_last();
        self.current.draw_itself(out)?;
        Ok(true)
    }
    fn move_tetromino_right(&mut self, out: &mut impl Write) -> Result<bool, io::Error> {
        let points = self.current.points();
        for point in points.iter() {
            if point[0] as u16 == self.grid.width - 1 || self.grid.data[point[1] as usize][point[0] as usize + 1].is_some() {
                return Ok(false);
            }
        }
        self.current.shift(Direction::Right);
        self.current.erase_last(out)?;
        self.current.set_last();
        self.current.draw_itself(out)?;
        Ok(true)
    }
    //TODO: kick wall, now can't rotate
    fn rotate_tetromino(&mut self, out: &mut impl Write) -> Result<bool, io::Error> {
        self.current.rotate();
        let points = self.current.points();
        for point in points.iter() {
            if point[0] as u16 >= self.grid.width || point[1] as u16 >= self.grid.height || self.grid.data[point[1] as usize][point[0] as usize].is_some() {
                self.current.reverse_back();
                return Ok(false);
            }
        }
        self.current.erase_last(out)?;
        self.current.set_last();
        self.current.draw_itself(out)?;
        Ok(true)
    }
}

//frame funcs
impl Game {
    fn draw_next_tip(&self, out: &mut impl Write) -> io::Result<()> {
        let &next = &self.queue.front().unwrap();
        self.current.erase_last(out)?;
        next.draw_position(out, super::constants::NEXT_TIP_POS, false)?;
        Ok(())
    }
    fn draw_score(&self, out: &mut impl Write) -> io::Result<()> {
        queue!(out,cursor::MoveTo(31,9),style::SetForegroundColor(Color::DarkYellow),Print(self.score),style::SetForegroundColor(Color::Reset))?;
        Ok(())
    }
    //fuse and draw
    fn fuse_tetromino(&mut self, out: &mut impl Write) -> io::Result<()> {
        self.current.erase_last(out)?;
        self.current.set_last();
        self.current.draw_itself(out)?;
        let points = self.current.points();
        for point in points.iter() {
            // print!("{:?}", point);
            self.grid.data[point[1] as usize][point[0] as usize] = Some(self.current.color());
        }
        self.check_full_row(out)?;
        Ok(())
        // print!("fuse fin");
    }
    fn draw_data(&self, out: &mut impl Write) -> io::Result<()> {
        // print!("draw data");
        for row in 0..self.grid.height {
            for col in 0..self.grid.width {
                if let Some(color) = self.grid.data[row as usize][col as usize] {
                    queue!(out,cursor::MoveTo(col * 2 + GAME_AREA_POSITION[0], row + GAME_AREA_POSITION[1]),style::SetForegroundColor(color),Print("■"),style::SetForegroundColor(Color::Reset))?;
                } else {
                    queue!(out,cursor::MoveTo(col * 2 + 1, row + 1),Print("  "))?;
                }
            }
        }
        Ok(())
    }
    fn check_full_row(&mut self, out: &mut impl Write) -> io::Result<()> {
        let points = self.current.points();
        let check_range: std::ops::RangeInclusive<usize> = points[0][1] as usize..=points[3][1] as usize;
        for row in check_range.rev() {
            // print!("{}'", row);
            let mut full = true;
            for col in 0..self.grid.width {
                // print!("{} ", col);
                if self.grid.data[row][col as usize].is_none() {
                    full = false;
                    break;
                }
            }
            if full {
                self.grid.data.remove(row);
                self.grid.data.insert(0, vec![None; self.grid.width as usize]);
                self.score += 1;
                self.draw_score(out)?;
                self.draw_data(out)?;
            }
        }
        Ok(())
    }
    // fn update_shadow(&mut self, out: &mut impl Write) {
    //
    //     self.shadow = self.current.clone();
    //     'points :loop {
    //         let points = self.shadow.points();
    //         for point in points.iter() {
    //             if point[1] as u16 == self.grid.height - 1 {
    //                 break 'points;
    //             }
    //             if self.grid.data[point[1] as usize + 1][point[0] as usize].is_some() {
    //                 break 'points;
    //             }
    //         }
    //         self.shadow.set_last();
    //         self.shadow.shift(Direction::Down);
    //     }
    //     //draw shadow
    //     self.shadow.shadow_draw_itself(out);
    // }
}


pub fn start(out: &mut impl Write) -> io::Result<()> {
    let mut game_instance = Game::new(GameMode::Normal, 0);
    frame::draw_frame(out)?;
    game_instance.game_loop(out)?;
    Ok(())
}

