// pager/src/ui

use crate::widget::{Bounds, Dimension, Position};
use crate::tag::{self, Tag};
use crate::view::{self, Dialog, DialogMsg, Tab, TabMsg, ViewMsg, View, Action};
use crossterm::{terminal, QueueableCommand};
use crossterm::style::{Color, Colors};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub enum CurrentView {
    Base(View),
    Dialog(View),
}

#[derive(Clone, Debug)]
pub struct RootView {
    size:      Dimension,
    view:      CurrentView,
    tablist:   Vec<Tab>,
    dialog:    Dialog,
    tabview:   String,
    history:   String,
    bookmarks: String,
} 
impl RootView {
    pub fn new(path: &String, w: usize, h: usize) -> Self {
        let mut tablist: Vec<Tab> = vec![];
        let size = Dimension {w: w, h: h};
        tablist.push(Tab::new(&path, size.clone()));
        Self {
            size:      size,
            view:      CurrentView::Base(View::Tab(0)),
            tablist:   tablist,
            tabview:   String::from(""),
            history:   String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.view {
            CurrentView::Base(v) => match v {
                View::Tab(i) => self.tablist[*i].view(stdout),
                _            => Ok(()),
            }
            CurrentView::Dialog(_) => self.dialog.view(&stdout),
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
            }) => 
                match &self.view {
                    CurrentView::Base(v) => 
                        match v {
                            View::Tab(i)    => self.updatetab(*i, keycode),
                            View::History   => self.updatehistory(keycode),
                            View::Bookmarks => self.updatebookmarks(keycode),
                            View::TabView   => self.updatetabview(keycode),
                            View::Quit      => false,
                        }
                    CurrentView::Dialog(v) => {
                        match self.dialog.update(keycode) {
                            Some(DialogMsg::Proceed(_)) => {
                                self.view = CurrentView::Base(v.clone());
                                true
                            }
                            _ => false,
                        }
                    }
                }
            _ => false,
        }
    }
    pub fn updatetab(&mut self, index: usize, keycode: KeyCode) -> bool {
        match self.tablist[index].update(keycode) {
            Some(msg) => {
                match msg {
                    TabMsg::Msg(ViewMsg::Dialog(d)) => {
                        self.dialog = d;
                        self.view   = CurrentView::Dialog(View::Tab(index));
                    }
                    TabMsg::Msg(ViewMsg::Base(View::Quit)) => {
                        self.view = CurrentView::Base(View::Quit);
                    }
                    _ => {}
                }
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
            CurrentView::Base(View::Quit) => true,
            _          => false,
        }
    }
} 
