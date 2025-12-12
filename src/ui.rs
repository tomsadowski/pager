// pager/src/ui

use crate::{
    reader::Reader,
    util::Bounds,
    util::Dimension,
    util::Location,
    util::GetColors,
    tag::TextTag, 
    tag::TaggedText
};
use crossterm::{
    style::Color, 
    style::Colors, 
    terminal,
    QueueableCommand,
    event::{Event, KeyEvent, KeyEventKind, KeyCode}};
use std::io::{self, Stdout};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

impl GetColors for TextTag {
    fn getcolors(&self) -> Colors {
        match self {
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
}

#[derive(Clone, PartialEq, Debug)]
pub enum Message {
    Code(char),
    Resize(usize, usize),
    Enter,
}

#[derive(Clone, Debug)]
pub struct UI {
    quit:      bool,
    tagreader: Reader<TextTag>,
    size:      Dimension,
} 
impl UI {
    pub fn new(text: String, w: u16, h: u16) -> Result<Self, String> {
        let text = TaggedText::parse_doc(text.lines().collect())?
            .iter()
            .map(|g| (g.tag.clone(), g.text.to_string()))
            .collect();

        let size   = Dimension {w: usize::from(w), h: usize::from(h)};
        let bounds = Bounds {loc: Location {x: 0, y: 0}, dim: size.clone()};

        Ok(Self {
            quit:      false,
            tagreader: Reader::new(text, bounds.clone()),
            size:      size,
        })
    }
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Enter => {
            }
            Message::Resize(w, h) => {
                self.size = Dimension {w: w, h: h};
                self.tagreader.resize(
                    Bounds {
                        loc: Location {x: 0, y: 0}, 
                        dim: self.size.clone()});
            }
            Message::Code(c) => {
                match c {
                    UP   => self.tagreader.movecursorup(),
                    DOWN => self.tagreader.movecursordown(),
                    QUIT => self.quit = true,
                    _ => {}
                }
            }
        }
    }
    pub fn view(&mut self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;
        self.tagreader.view(stdout)
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
            Event::Resize(y, x)  => Some(Message::Resize(usize::from(y), usize::from(x))),
            _                    => None
        }
    }
    pub fn quit(&self) -> bool {
        self.quit
    }
} 
