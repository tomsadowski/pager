// pager/src/ui

use crate::{
    reader::Reader,
    tag::{TextTag, TaggedText},
};
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

#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Code(char),
    Resize(u16, u16),
    Enter,
}
#[derive(Clone, Debug)]
pub struct UI {
    quit:      bool,
    dialog:    Option<String>,
    tagreader: Reader<TextTag>,
} 
impl UI {
    pub fn new(text: String, x: u16, y: u16) -> Result<Self, String> {
        let text = 
            Self::gettuples(&TaggedText::parse_doc(text.lines().collect())?);
        Ok(Self {
            quit:      false,
            dialog:    None,
            tagreader: Reader::new(text, x, y),
        })
    }
    pub fn gettuples(lines: &Vec<TaggedText>) -> Vec<(TextTag, Colors, String)> {
        lines
            .iter()
            .map(|g| (g.tag.clone(), Self::getcolors(&g.tag), g.text.to_string()))
            .collect()
    }
    pub fn getcolors(data: &TextTag) -> Colors {
        match data {
            TextTag::Heading => Colors::new(
                    Color::Rgb {r: 225,  g: 105,  b:  180},
                    Color::Rgb {r:   0,  g:   0,  b:    0},
                ),
            TextTag::Text => Colors::new(
                    Color::Rgb {r: 225,  g: 180,  b: 105},
                    Color::Rgb {r:   0,  g:   0,  b:   0},
                ),
            TextTag::Link(_) => Colors::new(
                    Color::Rgb {r: 180,  g: 105,  b: 225},
                    Color::Rgb {r:   0,  g:   0,  b:   0},
                ),
        }
    }
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Resize(x, y) => 
                self.tagreader.resize(x, y),
            Message::Enter => {
                match &self.dialog {
                    Some(text) => self.dialog = None,
                    _          => self.dialog = None,
                }
            }
            Message::Code(c) => {
                match c {
                    UP   => self.tagreader.mvcursorup(),
                    DOWN => self.tagreader.mvcursordown(),
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
            None => self.tagreader.view(stdout),
        }
    }
    // given a relevant Event, return some Message
    pub fn getupdate(event: Event) -> Option<Message> {
        match event {
            Event::Key(keyevent) => 
                match keyevent {
                    KeyEvent {
                        code: KeyCode::Char(c),
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        Some(Message::Code(c))
                    }
                    KeyEvent {
                        code: KeyCode::Enter,
                        kind: KeyEventKind::Press,
                        ..
                    } => {
                        Some(Message::Enter)
                    }
                    _ => None
                }
            Event::Resize(y, x)  => Some(Message::Resize(y, x)),
            _                    => None
        }
    }
    pub fn quit(&self) -> bool {
        self.quit
    }
} 
