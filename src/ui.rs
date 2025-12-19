// pager/src/ui

use crate::util::{Rect, View};
use crate::tabs::{TabMgr};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct UI {
    rect: Rect,
    view: View,
    tabs: TabMgr,
    history: String,
    bookmarks: String,
} 
impl UI {
    pub fn new(path: &str, w: u16, h: u16) -> Self {
        let rect = Rect::new(0, 0, w, h);
        Self {
            tabs: TabMgr::new(&rect, path),
            rect: rect,
            view: View::Tab,
            history: String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.view {
            View::Tab => self.tabs.view(stdout),
            _ => Ok(()),
        }?;
        stdout.flush()
    }
    fn resize(&mut self, w: u16, h: u16) {
        self.rect = Rect::new(0, 0, w, h);
        self.tabs.resize(&self.rect);
    }
    pub fn update(&mut self, event: Event) -> bool {
        match event {
            Event::Resize(w, h) => {
                self.resize(w, h); 
                true
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press, 
                ..
            }) => {
                self.view = View::Quit;
                true
            }
            Event::Key(KeyEvent {
                code: keycode, 
                kind: KeyEventKind::Press, 
                ..
            }) => 
                match &self.view {
                    View::Tab => self.tabs.update(&keycode),
                    _ => false,
                }
            _ => false,
        }
    }
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _ => false,
        }
    }
} 
