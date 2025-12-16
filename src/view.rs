// pager/src/view

use crate::widget::{Bounds, Dimension, Position, Selector};
use crate::dialog::{Dialog, Action, DialogMsg};
use crate::tag::{self, Tag};
use crossterm::{QueueableCommand, cursor, terminal, style};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::{env, fs};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum View {
    Tab(usize),
    Dialog,
    TabView,
    History,
    Bookmarks,
    Quit,
}
#[derive(Clone, Debug)]
pub enum ViewMsg {
    Stay,
    Switch(View),
    MakeDialog(Dialog),
}
#[derive(Clone, Debug)]
pub enum TabMsg {
    CycleLeft,
    CycleRight,
    Msg(ViewMsg),
}
#[derive(Clone, Debug)]
pub struct Tab {
    path: String,
    size: Dimension,
    main: Selector<Tag>,
}
impl Tab {
    pub fn new(path: &String, dim: Dimension) -> Self {
        let src    = fs::read_to_string(&path).unwrap();
        let text   = tag::parse_doc(src.lines().collect());
        let bounds = Bounds {
            pos: Position  {x: 0, y: 1}, 
            dim: Dimension {w: dim.w, h: dim.h - 1},
        };
        Self {
            size: dim,
            path: path.clone(),
            main: Selector::new(text, true, bounds.clone()),
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
        self.size = dim.clone();
        self.main.resize(
            Bounds {
                pos: Position  {x: 0, y: 1}, 
                dim: Dimension {w: dim.w, h: dim.h - 1}
            }
        );
    }
    pub fn update(&mut self, keycode: KeyCode) -> Option<TabMsg> {
        match keycode {
            KeyCode::Char('q') => {
                Some(TabMsg::Msg(ViewMsg::Switch(View::Quit)))
            }
            KeyCode::Char('i') => {
                self.main.movecursordown();
                Some(TabMsg::Msg(ViewMsg::Stay))
            }
            KeyCode::Char('o') => {
                self.main.movecursorup();
                Some(TabMsg::Msg(ViewMsg::Stay))
            }
            KeyCode::Enter  => {
                match self.main.selectundercursor() {
                    Tag::Text => {
                        let dialog = 
                            Dialog::new(
                                Action::Acknowledge,
                                String::from("you've selected plain old text."),
                                self.size.clone());
                        Some(TabMsg::Msg(ViewMsg::MakeDialog(dialog)))
                    }
                    Tag::Heading => {
                        let dialog = 
                            Dialog::new(
                                Action::Input(String::from("type something?")),
                                String::from("here: "),
                                self.size.clone());
                        Some(TabMsg::Msg(ViewMsg::MakeDialog(dialog)))
                    }
                    Tag::Link(l) => {
                        let dialog = 
                            Dialog::new(
                                Action::FollowPath(l.to_string()),
                                String::from("you've selected a path."),
                                self.size.clone());
                        Some(TabMsg::Msg(ViewMsg::MakeDialog(dialog)))
                    }
                }
            }
            _ => None,
        }
    }
}
