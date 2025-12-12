// pager/src/main

#![allow(unused_variables)]
#![allow(dead_code)]

mod ui;
mod tag;
mod reader;
mod util;

use crate::ui::UI;
use crossterm::{QueueableCommand, terminal, cursor, event};
use std::io::{self, stdout, Write};
use std::{env, fs};

fn main() -> io::Result<()> {
    // set up
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        panic!("supply path as arg")
    };
    let text       = fs::read_to_string(&path)?;
    let size       = terminal::size()?;
    let mut ui     = UI::new(text, size.0, size.1).unwrap();
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::Show)?;
    stdout.flush()?;
    // main loop
    while !ui.quit() {
        ui.view(&stdout)?;
        if let Some(msg) = UI::getupdate(event::read()?) {
            ui.update(msg);
        }
    }
    // clean up
    terminal::disable_raw_mode()?;
    stdout
        .queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}
