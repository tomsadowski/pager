// main

#![allow(unused_variables)]
#![allow(dead_code)]

mod model;
mod msg;

use crate::{
    model::Model,
    msg::Message,
};
use crossterm::{
    QueueableCommand, terminal, cursor, event
};
use std::io::{
    self, stdout, Write
};
use std::{
    env, fs
};


fn main() -> io::Result<()> 
{
    // set up
    let args: Vec<String> = env::args().collect();
    let Some(path) = args.get(1) else {
        panic!("supply path as arg")
    };
    terminal::enable_raw_mode()?;
    let text       = fs::read_to_string(&path)?;
    let mut model  = Model::init(&text, terminal::size()?);
    let mut stdout = stdout();
    stdout
        .queue(terminal::EnterAlternateScreen)?
        .queue(terminal::DisableLineWrap)?
        .queue(cursor::Show)?;


    // main loop
    while !model.quit()
    {
        model.view(&stdout)?;

        if let Some(msg) = Message::from_event(event::read()?) 
        {
            model.update(msg);
        }
    }


    // clean up
    terminal::disable_raw_mode()?;
    stdout
        .queue(terminal::LeaveAlternateScreen)?;
    stdout.flush()?;
    Ok(())
}
