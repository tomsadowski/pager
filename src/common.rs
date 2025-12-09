// pager/src/common

use crate::{
    tomtext::{TomTextLine, TomTextData}};
use crossterm::{
    style::{Colors, Color},
    event::{Event, KeyEvent, KeyEventKind, KeyCode}};

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
            Event::Key(keyevent) => Self::from_key_event(keyevent),
            Event::Resize(y, x)  => Some(Self::Resize(y, x)),
            _                    => None
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
pub fn get_tuples(lines: &Vec<TomTextLine>) -> Vec<(Colors, String)> {
    lines
        .iter()
        .map(|g| (get_colors(&g.data), g.text.to_string()))
        .collect()
}
pub fn get_colors(data: &TomTextData) -> Colors {
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
