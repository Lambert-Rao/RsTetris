use crossterm::{cursor, execute, queue, style, style::Color, style::SetBackgroundColor, Command, Result};
use base::tetromino;
use std::io::{stdout, Write};
use crossterm::style::Print;
use crate::base::tetromino::Tetromino;

mod base;

const MENU: &str = r#"Tetris
   "#;



fn main() -> Result<()> {
    let mut stdout = stdout();
    loop
    {
        base::menu::show_menu(&mut stdout)?;
        use base::menu::MenuOption;
        let opt = base::menu::select(&mut stdout)?;
        match opt {
            MenuOption::Start => {
                base::game::start(&mut stdout);
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

    base::frame::draw_frame(&mut stdout);


    execute!(stdout, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}