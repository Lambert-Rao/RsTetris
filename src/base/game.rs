use std::io::Write;
use crate::base::tetromino::Tetromino;
use super::frame;
use std::collections::LinkedList;
use std::thread::sleep;
use crossterm::{queue, style::Print, cursor};

enum GameMode {
    Normal
}

struct Game {
    mode: GameMode,
    difficulty: u8,
    queue: LinkedList<Tetromino>,
    score: u64
}

impl Game {
    fn new(out: &mut impl Write, mode: GameMode, difficulty: u8) -> Game {
        let n = Self {
            mode,
            difficulty,
            queue: Default::default(),
            score:0
        };
        frame::draw_frame(out);
        n
    }
    fn game_loop(&mut self, out: &mut impl Write) {
        self.queue.push_back(Tetromino::new());
        loop {
            //make this asnc
            self.queue.push_back(Tetromino::new());
            let current = self.queue.pop_front().expect("can not get a tetromino");
            self.draw_next_tip(out);
            out.flush();

        }
    }
}

impl Game {
    fn draw_next_tip(&self, out: &mut impl Write) {
        let &next = &self.queue.front().unwrap();
        for i in 0..4 {
            for j in 0..4 {
                queue!(out, cursor::MoveTo(26 + (j*2) as u16, 2 + i as u16),Print("  "));
            }
        }
        next.draw(out, [26, 2]);
    }
    fn draw_score(&self, out: &mut impl Write){

    }
}


pub fn start(out: &mut impl Write) {
    let mut game_instance = Game::new(out, GameMode::Normal, 0);
    game_instance.game_loop(out);
}

