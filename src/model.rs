// pager/src/model

use crate::{
    textview::TextView,
    tomtext::{TomTextLine, TomTextData}};
use crossterm::{
    style::{self, Colors, Color},
    QueueableCommand, cursor,
    event::{Event, KeyEvent, KeyEventKind, KeyCode}};
use std::io::{self, Write, Stdout};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

fn get_tuples(lines: &Vec<TomTextLine>) -> Vec<(Colors, String)> {
    lines
        .iter()
        .map(|g| (get_colors(&g.data), g.text.to_string()))
        .collect()
}
fn get_colors(data: &TomTextData) -> Colors {
    match data {
        TomTextData::Heading => Colors::new(
                Color::Rgb {r: 225,  g: 105,  b:  180},
                Color::Rgb {r:   0,  g:   0,  b:    0},
            ),
        TomTextData::Text => Colors::new(
                Color::Rgb {r: 225,  g: 180,  b: 105},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
        TomTextData::Link(_) => Colors::new(
                Color::Rgb {r: 180,  g: 105,  b: 225},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Code(char),
    Resize(u16, u16),
    Enter,
}
impl Message {
    // given a relevant Event, return some Message
    pub fn from_event(event: Event) -> Option<Self> {
        match event {
            Event::Key(keyevent) => 
                Self::from_key_event(keyevent),
            Event::Resize(y, x)  => 
                Some(Self::Resize(y, x)),
            _ => None
        }
    }
    // given a relevant KeyEvent, return some Message
    fn from_key_event(keyevent: KeyEvent) -> Option<Self> {
        match keyevent {
            KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                ..
            } => {
                Some(Self::Code(c))
            }
            KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                ..
            } => {
                Some(Self::Enter)
            }
            _ => None
        }
    }
}
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
                    Some(text) => 
                        self.dialog = None,
                    None => {
                        self.dialog = Some(
                            format!("{:?}", 
                                self.text_data[
                                    self.text_view
                                        .get_index_under_cursor()
                                ]
                            )
                        );
                    }
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
