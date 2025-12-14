// pager/src/ui

use crate::selector::Selector;
use crate::util::{Bounds, Dimension, Position};
use crate::tag::{self, Tag};

use crossterm::{terminal, QueueableCommand};
use crossterm::style::{Color, Colors};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};

use std::io::{self, Stdout};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';

#[derive(Clone, Debug)]
pub enum Action {
    SurrenderPrivacy,
    FollowPath,
}
#[derive(Clone, Debug)]
pub enum View {
    Quit,
    TabView,
    Tab(usize),
    Dialog(Action),
    History,
    Bookmarks,
}

#[derive(Clone, Debug)]
pub struct Data {
    pub size:      Dimension,
    pub view:      View,

    pub dialog:    String,
    pub tablist:   Vec<Selector<Tag>>,
    pub tabview:   String,
    pub history:   String,
    pub bookmarks: String,
} 
impl Data {
    pub fn new(text: String, w: usize, h: usize) -> Result<Self, String> {
        let mut tablist: Vec<Selector<Tag>> = vec![];
        let size     = Dimension {w: w, h: h};
        let bounds   = Bounds {pos: Position {x: 0, y: 0}, dim: size.clone()};
        let text     = tag::parse_doc(text.lines().collect())?;
        tablist.push(Selector::new(text, bounds.clone()));
        Ok(
            Self {
                size:      size,
                view:      View::Tab(0),
                tablist:   tablist,
                dialog:    String::from(""),
                tabview:   String::from(""),
                history:   String::from(""),
                bookmarks: String::from(""),
            }
        )
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        match self.view {
            View::Tab(i) => self.tablist[i].view(stdout),
            _               => Ok(()),
        }
    }
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Key(KeyEvent {
                code: keycode, 
                kind: KeyEventKind::Press, 
                ..
            }) => {
                match (keycode, &self.view) {
                    (KeyCode::Char('i'), View::Tab(i)) => {
                        self.tablist[*i].movecursordown();
                        true
                    }
                    (KeyCode::Char('o'), View::Tab(i)) => {
                        self.tablist[*i].movecursorup();
                        true
                    }
                    (KeyCode::Enter, _) => {
                        true
                    }
                    (KeyCode::Char('q'), _) => {
                        self.view = View::Quit;
                        true
                    }
                    _ => false,
                }
            }
            Event::Resize(w, h) => {
                self.size = Dimension {w: usize::from(w), h: usize::from(h)};
                self.tablist[0].resize(
                    Bounds {
                        pos: Position {x: 0, y: 0}, 
                        dim: self.size.clone()});
                true
            }
            _ => false,
        }
    }
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _          => false,
        }
    }
} 
