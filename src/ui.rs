// pager/src/ui

use crate::widget::{Dimension};
use crate::view::{View};
use crate::tabs::{TabMgr};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct UI {
    size: Dimension,
    view: View,
    tabs: TabMgr,
    history: String,
    bookmarks: String,
} 
impl UI {
    pub fn new(path: &str, w: usize, h: usize) -> Self {
        let size = Dimension {w: w, h: h};
        let tabs = TabMgr::new(path, &size);
        Self {
            size: size,
            view: View::Tab,
            tabs: tabs,
            history: String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.view {
            View::Tab => self.tabs.view(stdout),
            _ => Ok(()),
        }?;
        stdout.flush()?;
        Ok(())
    }
    fn resize(&mut self, w: u16, h: u16) {
        self.size = Dimension {w: usize::from(w), h: usize::from(h)};
        self.tabs.resize(&self.size);
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
                    View::Tab => self.updatetabs(keycode),
                    View::History => self.updatehistory(keycode),
                    View::Bookmarks => self.updatebookmarks(keycode),
                    View::Quit => false,
                }
            _ => false,
        }
    }
    pub fn updatetabs(&mut self, keycode: KeyCode) -> bool {
        self.tabs.update(keycode)
    }
    pub fn updatehistory(&mut self, keycode: KeyCode) -> bool {
        false
    }
    pub fn updatebookmarks(&mut self, keycode: KeyCode) -> bool {
        false
    }
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _ => false,
        }
    }
} 
