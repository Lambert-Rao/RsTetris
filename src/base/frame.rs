use std::collections::LinkedList;
use crossterm::{cursor, queue, terminal, style, style::Print, style::Color, style::SetBackgroundColor, Command, Result, execute};
use std::io::{Write, self};
use std::thread::sleep;

pub fn draw_frame(out: &mut impl Write) -> io::Result<()> {
    queue!(out, terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0,0),
    cursor::Hide,
    )?;
    for _ in 0..21 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    queue!(out, cursor::MoveToNextLine(1))?;
    for _ in 0..21 {
        queue!(out,
        Print("🞐".to_string()),
            cursor::MoveToColumn(40),
            Print("🞐".to_string()),
            cursor::MoveToNextLine(1))?;
    }
    for _ in 0..21 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    queue!(out,cursor::MoveTo(22,1))?;
    for _ in 0..21 {
        queue!(out,
        Print("🞐".to_string()),
            cursor::MoveToNextLine(1),
        cursor::MoveToColumn(22))?;
    }
    queue!(out,cursor::MoveTo(22,8));
    for _ in 0..9{
        queue!(out,Print("🞐 ".to_string()));
    }
    queue!(out,cursor::MoveTo(24,9),style::SetForegroundColor(Color::DarkYellow),Print("Score:"),style::SetForegroundColor(Color::Reset));
    queue!(out,cursor::MoveTo(22,10));
    for _ in 0..9{
        queue!(out,Print("🞐 ".to_string()));
    }
    out.flush()?;
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}

type PointState = Option<Color>;
struct Matrix {
    data: LinkedList<Vec<PointState>>,
    width: usize,
    height: usize,
}