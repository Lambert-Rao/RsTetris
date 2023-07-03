use std::io;
use std::io::Write;
use crossterm::*;
use crossterm::style::{self, *};
use crossterm::event::{ *};


const TITLE: &str =
"██████╗ ███████╗████████╗███████╗████████╗██████╗ ██╗███████╗
██╔══██╗██╔════╝╚══██╔══╝██╔════╝╚══██╔══╝██╔══██╗██║██╔════╝
██████╔╝███████╗   ██║   █████╗     ██║   ██████╔╝██║███████╗
██╔══██╗╚════██║   ██║   ██╔══╝     ██║   ██╔══██╗██║╚════██║
██║  ██║███████║   ██║   ███████╗   ██║   ██║  ██║██║███████║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝";


pub fn show_menu(out: &mut impl Write) -> io::Result<()> {
    draw_menu_frame(out)?;
    //Menu
    queue!(out,cursor::MoveTo(15,15))?;
    draw_menu(out,0)?;
    out.flush()?;
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}
pub fn select(out: &mut impl Write) -> io::Result<MenuOption> {
    let mut user_cursor = 0;
    execute!(out, cursor::MoveTo(15, 15),cursor::Hide)?; // move to the first menu item
    terminal::enable_raw_mode()?;
    loop {
        match read()? {
            Event::Key(KeyEvent {
                           code: KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K'),
                           kind: KeyEventKind::Press,
                           ..
                       }) => {
                if user_cursor > 0 {
                    user_cursor -= 1;
                }
                draw_menu(out, user_cursor)?;
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J'),
                           kind: KeyEventKind::Press,
                           ..
                       }) => {
                if user_cursor < (MenuOption::MENU_STR().len() - 1) as u8 {
                    user_cursor += 1;
                }
                draw_menu(out, user_cursor)?;
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Enter | KeyCode::Char(' ')|KeyCode::Char('E')|KeyCode::Char('e'),
                           kind: KeyEventKind::Press,
                           ..
                       }) => {
                match user_cursor {
                    0 => {
                        terminal::disable_raw_mode()?;
                        return Ok(MenuOption::Start);
                    }
                    1 => {
                        terminal::disable_raw_mode()?;
                        return Ok(MenuOption::Settings);
                    }
                    2 => {
                        terminal::disable_raw_mode()?;
                        return Ok(MenuOption::About);
                    }
                    3 => {
                        terminal::disable_raw_mode()?;
                        return Ok(MenuOption::Quit);
                    }
                    _ => { panic!("Invalid menu option") }
                }
            }
            Event::Key(KeyEvent {
                           code: KeyCode::Esc,
                           kind: KeyEventKind::Press,
                           ..
                       }) => {
                terminal::disable_raw_mode()?;
                return Ok(MenuOption::Quit);
            }
            _ => {}
        }
    }
}

fn draw_block(out: &mut impl Write) -> io::Result<()> {
    //draw a box around the menu
    queue!(out, terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0,0),
    cursor::Hide,
    )?;
    for _ in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    queue!(out, cursor::MoveToNextLine(1))?;
    for _ in 0..35 {
        queue!(out,
        Print("🞐".to_string()),
            cursor::MoveToColumn(68),
            Print("🞐".to_string()),
            cursor::MoveToNextLine(1))?;
    }
    for _ in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    out.flush()?;
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}

pub enum MenuOption {
    Start,
    Settings,
    About,
    Quit,
}

impl MenuOption {
    fn MENU_STR() -> [&'static str; 4] {
        match  std::env::var("DISPLAY") {
            Err(_) => ["Game Start", "Settings", "About", "Quit"],
            Ok(_) => ["𝔾𝕒𝕞𝕖 𝕊𝕥𝕒𝕣𝕥", "𝕊𝕖𝕥𝕥𝕚𝕟𝕘𝕤", "𝔸𝕓𝕠𝕦𝕥", "ℚ𝕦𝕚𝕥"],
        }
    }
}

fn draw_menu_frame(out: &mut impl Write) -> io::Result<()>{
    draw_block(out)?;
    //Title
    queue!(out,cursor::MoveTo(5,2))?;
    for line in TITLE.split('\n') {
        queue!(out, Print(line), cursor::MoveDown(1),cursor::MoveToColumn(5))?;
    }
    //Line
    queue!(out,cursor::MoveToColumn(0),cursor::MoveToNextLine(1))?;
    for _ in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}

fn draw_menu (out: &mut impl Write,opt:u8) -> io::Result<()>{
    //Menu
    queue!(out,cursor::MoveTo(15,15),style::SetForegroundColor(Color::Reset))?;
    for i in 0..MenuOption::MENU_STR().len() {
        if i == opt as usize {
            queue!(out,style::SetForegroundColor(Color::Red), Print("🞂\t"),Print(MenuOption::MENU_STR()[i]), cursor::MoveDown(3),cursor::MoveToColumn(15),
            style::SetForegroundColor(Color::Reset),)?;
        }
        else {
            queue!(out, Print("🞅\t"),Print(MenuOption::MENU_STR()[i]), cursor::MoveDown(3),cursor::MoveToColumn(15))?;
        }
    }
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}