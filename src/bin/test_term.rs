use std::io::{stdout, Write};

use crossterm::{cursor, execute, queue, QueueableCommand, style};
use crossterm::cursor::MoveToNextLine;
use crossterm::style::{Attribute, Color, Stylize};
use crossterm::terminal::{Clear, ClearType, size};

fn main() {
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All));
    let s = size().unwrap();


    queue!(stdout, cursor::MoveTo(5,5));
    queue!(stdout, style::PrintStyledContent( format!("width {}", s.0).with(Color::Rgb {r:120,g:0,b:0})));
    //queue!(stdout, cursor::MoveTo(5,6));
    queue!(stdout, style::PrintStyledContent( format!("height {}", s.1).with(Color::Rgb {r:0,g:0,b:125})));
    execute!(stdout, MoveToNextLine(1));
    queue!(stdout, style::Print("fuck you every day"));

    stdout.flush();

}