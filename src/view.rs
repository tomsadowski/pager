
use crate::widget::{Bounds, Dimension, Position, Selector};
use crate::tag::{self, Tag};
use crossterm::{QueueableCommand, cursor, terminal, style};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::{env, fs};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub struct Tab {
    pub path: String,
    pub main: Selector<Tag>,
}
impl Tab {
    pub fn new(path: &String, dim: Dimension) -> Self {
        let src    = fs::read_to_string(&path).unwrap();
        let bounds = Bounds {pos: Position {x: 0, y: 1}, dim: dim};
        let text   = tag::parse_doc(src.lines().collect());
        Self {
            path: path.clone(),
            main: Selector::new(text, bounds.clone()),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::Print(self.path.as_str()))?;
        self.main.view(stdout)
    }
    pub fn resize(&mut self, dim: Dimension) {
        self.main.resize(Bounds {pos: Position {x: 0, y: 1}, dim: dim});
    }
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: keycode, 
                kind: KeyEventKind::Press, 
                ..
            }) => {
                match keycode {
                    KeyCode::Char('i') => {
                        self.main.movecursordown();
                        true
                    }
                    KeyCode::Char('o') => {
                        self.main.movecursorup();
                        true
                    }
                    KeyCode::Enter  => {
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
}
