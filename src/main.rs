// pager/src/main

#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod ui;
mod tag;
mod selector;
mod util;
mod interface;

use crate::ui::Data;
use crossterm::{QueueableCommand, terminal, cursor, event};
use std::io::{self, stdout, Write};
use std::{env, fs};

fn main() -> io::Result<()> {
    // set up
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        panic!("supply path as arg")
    };
    let text   = fs::read_to_string(&path)?;
    let (w, h) = terminal::size()?;
    let mut ui = Data::new(text, usize::from(w), usize::from(h)).unwrap();

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::Show)?;
    stdout.flush()?;

    ui.view(&stdout)?;

    // main loop
    while !ui.quit() {
        if ui.update(event::read()?) {
            ui.view(&stdout)?;
        }
    }

    // clean up
    terminal::disable_raw_mode()?;
    stdout.queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}
