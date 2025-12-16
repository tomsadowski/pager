// pager/src/ui

use crate::tag::{self, Tag};
use crate::widget::{Bounds, Dimension, Position};
use crate::view::{Tab, TabMsg, ViewMsg, View};
use crate::dialog::{Dialog, DialogMsg, Action};
use crossterm::{terminal, QueueableCommand};
use crossterm::style::{Color, Colors};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct UI {
    size:      Dimension,
    lastview:  View,
    curview:   View,
    tablist:   Vec<Tab>,
    dialog:    Dialog,
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
            lastview:  View::Tab(0),
            curview:   View::Tab(0),
            dialog:    Dialog::default(),
            tablist:   tablist,
            tabview:   String::from(""),
            history:   String::from(""),
            bookmarks: String::from(""),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        match &self.curview {
            View::Tab(i) => self.tablist[*i].view(stdout),
            View::Dialog => self.dialog.view(&stdout),
            _            => Ok(()),
        }?;
        stdout.flush()?;
        Ok(())
    }
    fn resize(&mut self, w: u16, h: u16) {
        self.size = Dimension {w: usize::from(w), h: usize::from(h)};
        self.tablist[0].resize(self.size.clone());
        self.dialog.resize(self.size.clone());
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
                match &self.curview {
                    View::Tab(i)    => self.updatetab(*i, keycode),
                    View::History   => self.updatehistory(keycode),
                    View::Bookmarks => self.updatebookmarks(keycode),
                    View::TabView   => self.updatetabview(keycode),
                    View::Dialog    => self.updatedialog(keycode),
                    View::Quit      => false,
                }
            _ => false,
        }
    }
    pub fn updatetab(&mut self, index: usize, keycode: KeyCode) -> bool {
        match self.tablist[index].update(keycode) {
            Some(msg) => {
                match msg {
                    TabMsg::Msg(ViewMsg::MakeDialog(d)) => {
                        self.dialog = d;
                        self.lastview = self.curview.clone();
                        self.curview  = View::Dialog;
                    }
                    TabMsg::Msg(ViewMsg::Switch(View::Quit)) => {
                        self.curview = View::Quit;
                    }
                    _ => {}
                }
                true
            }
            None => false
        }
    }
    pub fn updatedialog(&mut self, keycode: KeyCode) -> bool {
        match self.dialog.update(keycode) {
            Some(DialogMsg::Proceed(_)) => {
                self.curview = self.lastview.clone();
                true
            }
            Some(_) => true,
            _       => false,
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
        match self.curview {
            View::Quit => true,
            _          => false,
        }
    }
} 
