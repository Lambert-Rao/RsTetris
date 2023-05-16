use std::io::{self, Write};
use crossterm::{*};
use crossterm::event::read;
use crossterm::style::Print;

pub fn settings(out: &mut impl Write) -> io::Result<()> {
    //clear screen
    execute!(out,cursor::MoveTo(2,10))?;
    for _ in 0..24 {
        for _ in 0..66 {
            queue!(out,style::Print(" "))?;
        }
        queue!(out,cursor::MoveToNextLine(1),cursor::MoveToColumn(2))?;
    }

    queue!(out,cursor::MoveTo(4,13))?;
    queue!(out,Print("Now Settings is not supported. Please wait for the next edition"))?;


    out.flush()?;

    terminal::enable_raw_mode()?;
    read()?;
    terminal::disable_raw_mode()?;
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}
