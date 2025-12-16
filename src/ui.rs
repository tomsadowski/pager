// pager/src/ui

use crate::widget::{Dimension};
use crate::view::{Tab, View};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct UI {
    size:      Dimension,
    view:      View,
    tablist:   Vec<Tab>,
    tabview:   String,
    history:   String,
    bookmarks: String,
} 
impl UI {
    pub fn new(path: &String, w: usize, h: usize) -> Self {
        let mut tablist: Vec<Tab> = vec![];
        let size = Dimension {w: w, h: h};
        tablist.push(Tab::new(&path, size.clone()));
        Self {
            size:      size,
            view:      View::Tab(0),
            tablist:   tablist,
            tabview:   String::from(""),
            history:   String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.view {
            View::Tab(i) => self.tablist[*i].view(stdout),
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
                    View::Tab(i)    => self.updatetab(*i, keycode),
                    View::History   => self.updatehistory(keycode),
                    View::Bookmarks => self.updatebookmarks(keycode),
                    View::TabView   => self.updatetabview(keycode),
                    View::Quit      => false,
                }
            _ => false,
        }
    }
    pub fn updatetab(&mut self, index: usize, keycode: KeyCode) -> bool {
        match self.tablist[index].update(keycode) {
            Some(_msg) => {
                true
            }
            None => false
        }
    }
    pub fn updatehistory(&mut self, keycode: KeyCode) -> bool {
        false
    }
    pub fn updatebookmarks(&mut self, keycode: KeyCode) -> bool {
        false
    }
    pub fn updatetabview(&mut self, keycode: KeyCode) -> bool {
        false
    }
    pub fn quit(&self) -> bool {
        match self.view {
            View::Quit => true,
            _          => false,
        }
    }
} 
