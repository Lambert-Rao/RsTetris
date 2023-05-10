use std::io::Write;
use crossterm::*;
use crossterm::style::{self, *};
use crossterm::event::{self, *};

const MENU: &str =
    "𝔾𝕒𝕞𝕖 𝕊𝕥𝕒𝕣𝕥\n𝕊𝕖𝕥𝕥𝕚𝕟𝕘𝕤\n𝔸𝕓𝕠𝕦𝕥";
const TITLE: &str =
"██████╗ ███████╗████████╗███████╗████████╗██████╗ ██╗███████╗
██╔══██╗██╔════╝╚══██╔══╝██╔════╝╚══██╔══╝██╔══██╗██║██╔════╝
██████╔╝███████╗   ██║   █████╗     ██║   ██████╔╝██║███████╗
██╔══██╗╚════██║   ██║   ██╔══╝     ██║   ██╔══██╗██║╚════██║
██║  ██║███████║   ██║   ███████╗   ██║   ██║  ██║██║███████║
╚═╝  ╚═╝╚══════╝   ╚═╝   ╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚══════╝";


pub fn show_menu(out: &mut impl Write) -> std::io::Result<()> {
    draw_block(out)?;
    //Title
    queue!(out,cursor::MoveTo(5,2))?;
    for line in TITLE.split('\n') {
        queue!(out, Print(line), cursor::MoveDown(1),cursor::MoveToColumn(5))?;
    }
    //Line
    queue!(out,cursor::MoveToColumn(0),cursor::MoveToNextLine(1))?;
    for i in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    //Menu
    queue!(out,cursor::MoveTo(15,15))?;
    for line in MENU.split('\n') {
        queue!(out, Print("🞐\t"),Print(line), cursor::MoveDown(3),cursor::MoveToColumn(15))?;
    }
    queue!(out,cursor::MoveTo(100,40))?;
    out.flush();
    Ok(())
}


fn draw_block(out: &mut impl Write) -> std::io::Result<()> {
    //draw a box around the menu
    queue!(out, terminal::Clear(terminal::ClearType::All),
    cursor::MoveTo(0,0),
    cursor::Hide,
    )?;
    for i in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    queue!(out, cursor::MoveToNextLine(1))?;
    for i in 0..35 {
        queue!(out,
        Print("🞐".to_string()),
            cursor::MoveToColumn(68),
            Print("🞐".to_string()),
            cursor::MoveToNextLine(1))?;
    }
    for i in 0..35 {
        queue!(out, Print("🞐 ".to_string()))?;
    }
    out.flush();
    Ok(())
}

pub fn select(out: &mut impl Write) -> std::io::Result<()> {
    let user_cursor = cursor::MoveTo(15,15);//menu postion
    loop {
        match read()? {
            Event::Key()
        }
    }
    out.flush();
    Ok(())
}

