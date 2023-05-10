
use crossterm::{cursor, execute, queue, style, style::Color, style::SetBackgroundColor, Command,Result};
use base::tetromino;
use std::io::{stdout, Write};
use crossterm::style::Print;

mod base;

const MENU:&str = r#"Tetris
   "#;

type PointState = Option<Color>;
struct Matrix{
    data:Vec<Vec<PointState>>,
    width: usize,
    height: usize
}


fn main()-> Result<()> {
    let mut stdout = stdout();
    base::menu::show_menu(&mut stdout)?;
    base::menu::select(&mut stdout)?;
    Ok(())
}