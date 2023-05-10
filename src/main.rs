use crossterm::{cursor, execute, queue, style, style::Color, style::SetBackgroundColor, Command, Result};
use base::tetromino;
use std::io::{stdout, Write};
use crossterm::style::Print;

mod base;

const MENU: &str = r#"Tetris
   "#;

type PointState = Option<Color>;

struct Matrix {
    data: Vec<Vec<PointState>>,
    width: usize,
    height: usize,
}


fn main() -> Result<()> {
    let mut stdout = stdout();
    loop
    {
        base::menu::show_menu(&mut stdout)?;
        use base::menu::MenuOption;
        let opt = base::menu::select(&mut stdout)?;
        match opt {
            MenuOption::Start => {
                panic!("Not implemented yet")
            }
            MenuOption::Settings => {}
            MenuOption::About => {
                base::about::about(&mut stdout)?;
            }
            MenuOption::Quit => {
                execute!(stdout, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
                return Ok(());
            }
        }
    }
    execute!(stdout, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}