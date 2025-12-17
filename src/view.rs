// pager/src/view

use crate::widget::{Bounds, Dimension, Position, Selector};
use crate::dialog::{Dialog, InputType, DialogMsg, Action};
use crate::tag::{self, Tag};
use crossterm::{QueueableCommand, cursor, terminal};
use crossterm::event::{KeyCode};
use crossterm::style::{self, Colors, Color};
use std::{fs};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum View {
    Tab(usize),
    TabView,
    History,
    Bookmarks,
    Quit,
}
#[derive(Clone, Debug)]
pub enum ViewMsg {
    None,
    FollowPath(String),
    Switch(View),
}
#[derive(Clone, Debug)]
pub enum TabMsg {
    CycleLeft,
    CycleRight,
    DeleteMe,
    FollowPath(String),
    Msg(ViewMsg),
}
#[derive(Clone, Debug)]
pub struct Tab {
    path:        String,
    size:        Dimension,
    dialogstack: Vec<Dialog>,
    main:        Selector<Tag>,
}
impl Tab {
    pub fn new(path: &String, dim: Dimension) -> Self {
        let src    = fs::read_to_string(&path).unwrap();
        let text   = tag::parse_doc(src.lines().collect());
        let bounds = Bounds {
            pos: Position  {x: 0, y: 2}, 
            dim: Dimension {w: dim.w, h: dim.h - 1},
        };
        Self {
            size: dim,
            path: path.clone(),
            dialogstack: vec![],
            main: Selector::new(text, true, bounds.clone()),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(style::SetColors(Colors::new(
                Color::Rgb {r: 180,  g: 180,  b: 180},
                Color::Rgb {r:   0,  g:   0,  b:   0})))?
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::Print(self.path.as_str()))?
            .queue(cursor::MoveTo(0, 1))?
            .queue(style::Print("----------------------------------------"))?
            .queue(cursor::MoveTo(0, 2))?;
        match self.dialogstack.last() {
            Some(d) => d.view(stdout),
            _       => self.main.view(stdout),
        }
    }
    pub fn resize(&mut self, dim: Dimension) {
        self.size = dim.clone();
        let bounds = 
            Bounds {
                pos: Position  {x: 0, y: 2}, 
                dim: Dimension {w: dim.w, h: dim.h - 1}
            };
        self.main.resize(bounds.clone());
        for d in self.dialogstack.iter_mut() {
            d.resize(bounds.clone());
        }
    }
    pub fn update(&mut self, keycode: KeyCode) -> Option<TabMsg> {
        if let Some(d) = self.dialogstack.last_mut() {
            match d.update(keycode) {
                Some(DialogMsg::Submit) => {
                    let msg = match (&d.action, &d.input) {
                        (Action::FollowPath(p), InputType::Choose((c, _))) => {
                            match c {
                                'y' => 
                                    Some(TabMsg::Msg(
                                            ViewMsg::FollowPath(p.clone()))),
                                _ => 
                                    Some(TabMsg::Msg(ViewMsg::None)),
                            }
                        }
                        (_, _) => 
                            Some(TabMsg::Msg(ViewMsg::None)),
                    };
                    self.dialogstack.pop();
                    return msg
                }
                Some(DialogMsg::Cancel) => {
                    self.dialogstack.pop();
                    return Some(TabMsg::Msg(ViewMsg::None))
                }
                Some(_) => {
                    return Some(TabMsg::Msg(ViewMsg::None))
                }
               _ => return None
            }
        }
        match keycode {
            KeyCode::Char('i') => {
                self.main.movecursordown();
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('o') => {
                self.main.movecursorup();
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('e') => {
                Some(TabMsg::CycleLeft)
            }
            KeyCode::Char('n') => {
                Some(TabMsg::CycleRight)
            }
            KeyCode::Enter => {
                let dialog = match self.main.selectundercursor() {
                    Tag::Text => {
                        Dialog::new(
                            Action::None,
                            String::from(""),
                            InputType::None,
                            Bounds::fromdimension(&self.size))
                    }
                    Tag::Heading => {
                        Dialog::new(
                            Action::None,
                            String::from("type here: "),
                            InputType::Input(vec![]),
                            Bounds::fromdimension(&self.size))
                    }
                    Tag::Link(l) => {
                        Dialog::new(
                            Action::FollowPath(l.to_string()),
                            String::from("follow the link?"),
                            InputType::Choose(('n', vec![
                                ('y', String::from("yes")), 
                                ('n', String::from("no"))])),
                            Bounds::fromdimension(&self.size))
                    }
                };
                self.dialogstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            _ => None,
        }
    }
}
