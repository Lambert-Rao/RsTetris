use crossterm::{cursor, execute, Result};
use std::io::{stdout};

mod base;
mod debug;


fn main() -> Result<()> {
    let mut stdout = stdout();
    loop
    {
        base::menu::show_menu(&mut stdout)?;
        use base::menu::MenuOption;
        let opt = base::menu::select(&mut stdout)?;
        match opt {
            MenuOption::Start => {
                base::game::start(&mut stdout)?;
            }
            MenuOption::Settings => {
                base::settings::settings(&mut stdout)?;
            }
            MenuOption::About => {
                base::about::about(&mut stdout)?;
            }
            MenuOption::Quit => {
                execute!(stdout, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
                return Ok(());
            }
        }
    }

}