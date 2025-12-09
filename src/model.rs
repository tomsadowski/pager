// pager/src/model

use crate::{
    common::{Message, get_tuples},
    textview::TextView,
    tomtext::{TomTextLine, TomTextData}};
use crossterm::{
    style,
    QueueableCommand, cursor};
use std::io::{self, Write, Stdout};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

#[derive(Clone, Debug)]
pub struct Model {
    quit:       bool,
    dialog:     Option<String>,
    text_data:  Vec<TomTextData>,
    text_view:  TextView,
} 
impl Model {
    pub fn new(text: String, size: (u16, u16)) -> Result<Self, String> {
        let lines = TomTextLine::parse_doc(text.lines().collect())?;
        Ok(Self {
            quit:      false,
            dialog:    None,
            text_data: lines.iter().map(|l| l.data.clone()).collect(),
            text_view: TextView::new(get_tuples(&lines), size.0, size.1),
        })
    }
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Resize(x, y) => 
                self.text_view.resize(x, y),
            Message::Enter => {
                match &self.dialog {
                    Some(text) => self.dialog = None,
                    _ => 
                        self.dialog = 
                            Some(
                                format!("{:?}", 
                                    self.text_data[self.text_view.get_index_under_cursor()])),
                }
            }
            Message::Code(c) => {
                match c {
                    UP   => self.text_view.move_cursor_up(),
                    DOWN => self.text_view.move_cursor_down(),
                    QUIT => self.quit = true,
                    _ => {}
                }
            }
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.dialog {
            Some(text) => {
                stdout
                    .queue(cursor::MoveTo(0, 0))?
                    .queue(style::Print(text))?;
                stdout.flush()?;
                Ok(())
            }
            None => self.text_view.view(stdout),
        }
    }
    pub fn quit(&self) -> bool {
        self.quit
    }
} 
