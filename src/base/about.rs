use std::io::{self,Write};
use crossterm::{*};
use crossterm::event::read;
use crossterm::style::Print;

pub fn about(out:&mut impl Write) -> io::Result<()>{
    //clear screen
    execute!(out,cursor::MoveTo(2,10))?;
    for _ in 0..24{
        for _ in 0..66{
            queue!(out,style::Print(" "))?;
        }
        queue!(out,cursor::MoveToNextLine(1),cursor::MoveToColumn(2))?;
    }

    queue!(out,cursor::MoveTo(4,13))?;
    for i in 0..5 {
        queue!(out,Print(INFO[i][0]),cursor::MoveToColumn(20))?;
        queue!(out,Print(INFO[i][1]),)?;
        queue!(out,cursor::MoveToNextLine(2),cursor::MoveToColumn(4))?;
    }


    out.flush()?;

    terminal::enable_raw_mode()?;
    read()?;
    terminal::disable_raw_mode()?;
    execute!(out, cursor::MoveTo(0,crossterm::terminal::size().unwrap().1))?;
    Ok(())
}


const INFO: [[&str;2];5] = [
    ["Version", "𝕧 𝟘.𝟙.𝟘"],
    ["Author", "𝕃𝕒𝕞𝕓𝕖𝕣𝕥 ℝ𝕒𝕠"],
    ["License", "𝕄𝕀𝕋 𝕃𝕚𝕔𝕖𝕟𝕤𝕖"],
    ["Dependency", "ᴄʀᴏꜱꜱᴛᴇʀᴍ  ʀᴀɴᴅ"],
    ["Repository", "https://github.com/Lambert-Rao/RsTetris"],
];