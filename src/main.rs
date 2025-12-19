// pager/src/main

#![allow(unused_variables)]
#![allow(dead_code)]

mod ui;
mod util;
mod tag;
mod widget;
mod tabs;

use crate::ui::UI;
use crossterm::{QueueableCommand, terminal, cursor, event};
use std::io::{self, stdout, Write};
use std::{env};

fn main() -> io::Result<()> {
    // set up
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        panic!("supply path as arg")
    };
    let (w, h) = terminal::size()?;
    let mut ui = UI::new(path, w, h);

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
