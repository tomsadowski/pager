// pager/src/ui

use crate::widget::{Bounds, Dimension, Position};
use crate::tag::{self, Tag};
use crate::view::{self, Tab};
use crossterm::{terminal, QueueableCommand};
use crossterm::style::{Color, Colors};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::io::{self, Write, Stdout};

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
    size:      Dimension,
    view:      View,
    tablist:   Vec<Tab>,
    tabview:   String,
    dialog:    String,
    history:   String,
    bookmarks: String,
} 
impl Data {
    pub fn new(path: &String, w: usize, h: usize) -> Self {
        let mut tablist: Vec<Tab> = vec![];
        let size = Dimension {w: w, h: h};
        tablist.push(Tab::new(&path, size.clone()));
        Self {
            size:      size,
            view:      View::Tab(0),
            tablist:   tablist,
            dialog:    String::from(""),
            tabview:   String::from(""),
            history:   String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match self.view {
            View::Tab(i) => self.tablist[i].view(stdout),
            _            => Ok(()),
        }?;
        stdout.flush()?;
        Ok(())
    }
    fn resize(&mut self, w: u16, h: u16) {
        self.size = Dimension {w: usize::from(w), h: usize::from(h)};
        self.tablist[0].resize(self.size.clone());
    }
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Resize(w, h) => {
                self.resize(w, h);
                true
            }
            Event::Key(KeyEvent {
                code: keycode, 
                kind: KeyEventKind::Press, 
                ..
            }) => {
                match keycode {
                    KeyCode::Char('q') => {
                        self.view = View::Quit;
                        true
                    }
                    _ => false,
                }
            }
            _ => {
                match &self.view {
                    View::Tab(i) => {
                        self.tablist[*i].update(event);
                        true
                    }
                    _ => false,
                }
            }
        }
    }
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _          => false,
        }
    }
} 
