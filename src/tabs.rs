// pager/src/tabs

use crate::widget::{Bounds, Dimension, Position, Selector};
use crate::view::{ViewMsg};
use crate::dialog::{Dialog, InputType, DialogMsg};
use crate::tag::{self, Tag};
use crossterm::{QueueableCommand, cursor, terminal};
use crossterm::event::{KeyCode};
use crossterm::style::{self, Colors, Color};
use std::{fs};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub struct TabMgr {
    dim: Dimension,
    bounds: Bounds,
    bannerstr: String,
    bannerline: String,
    bannerstrcolor: Colors,
    bannerlinecolor: Colors,
    curindex: usize,
    tabs: Vec<Tab>,
}
impl TabMgr {
    pub fn new(path: &str, dim: &Dimension) -> Self {
        let bounds = Bounds {
            pos: Position {x: 0, y: 2}, 
            dim: Dimension {w: dim.w, h: dim.h - 1},
        };
        let tab = Tab::new(path, &bounds);
        let curindex = 0;
        let bannerstr = Self::bannerstr(curindex, 1, path);
        let bannerline = Self::bannerline(&dim);
        Self {
            dim: dim.clone(),
            bounds: bounds.clone(),
            bannerstr: bannerstr,
            bannerline: bannerline,
            bannerstrcolor: Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0}),
            bannerlinecolor: Colors::new(
                Color::Rgb {r: 180, g: 180, b: 180},
                Color::Rgb {r: 0, g: 0, b: 0}),
            curindex: curindex,
            tabs: vec![tab],
        }
    }
    pub fn add(&mut self, path: &str) {
        self.tabs.push(Tab::new(path, &self.bounds));
    }
    pub fn resize(&mut self, dim: &Dimension) {
        self.dim = dim.clone();
        self.bannerline = Self::bannerline(&self.dim);
        self.bounds = Bounds {
            pos: Position  {x: 0, y: 2}, 
            dim: Dimension {w: dim.w, h: dim.h - 1}
        };
        for d in self.tabs.iter_mut() {
            d.resize(&self.bounds);
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .queue(style::SetColors(self.bannerstrcolor))?
            .queue(style::Print(self.bannerstr.as_str()))?
            .queue(cursor::MoveTo(0, 1))?
            .queue(style::SetColors(self.bannerlinecolor))?
            .queue(style::Print(&self.bannerline))?;
        self.tabs[self.curindex].view(stdout)
    }
    pub fn update(&mut self, keycode: KeyCode) -> bool {
        match self.tabs[self.curindex].update(keycode) {
            Some(msg) => {
                match msg {
                    TabMsg::Msg(ViewMsg::Go(p)) => {
                        self.tabs.push(Tab::new(&p, &self.bounds));
                        self.curindex = self.tabs.len() - 1;
                    }
                    TabMsg::DeleteMe => {
                        self.tabs.remove(self.curindex);
                        self.curindex = self.tabs.len() - 1;
                    }
                    TabMsg::CycleLeft => {
                        match self.curindex == 0 {
                            true => self.curindex = self.tabs.len() - 1,
                            false => self.curindex -= 1,
                        }
                    }
                    TabMsg::CycleRight => {
                        match self.curindex == self.tabs.len() - 1 {
                            true => self.curindex = 0,
                            false => self.curindex += 1,
                        }
                    }
                    _ => {},
                }
                let len = self.tabs.len();
                let path = &self.tabs[self.curindex].path;
                self.bannerstr = Self::bannerstr(self.curindex, len, path);
                self.bannerline = Self::bannerline(&self.dim);
                true
            }
            None => false,
        }
    }
    fn bannerstr(curindex: usize, totaltab: usize, path: &str) -> String {
        format!("{}/{}: {}", curindex + 1, totaltab, path)
    }
    fn bannerline(dim: &Dimension) -> String {
        String::from("-").repeat(dim.w)
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    None,
    GoTo,
    DeleteMe,
    Go(String),
}
#[derive(Clone, Debug)]
pub enum TabMsg {
    CycleLeft,
    CycleRight,
    DeleteMe,
    Go(String),
    Msg(ViewMsg),
}
#[derive(Clone, Debug)]
pub struct Tab {
    bounds: Bounds,
    pub path: String,
    dialogstack: Vec<Dialog<Action>>,
    page: Selector<Tag>,
}
impl Tab {
    pub fn new(path: &str, bounds: &Bounds) -> Self {
        let src = fs::read_to_string(&path).unwrap();
        let text = tag::parse_doc(src.lines().collect());
        Self {
            bounds: bounds.clone(),
            path: String::from(path),
            dialogstack: vec![],
            page: Selector::new(text, true, bounds.clone()),
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout.queue(cursor::MoveTo(0, self.bounds.pos.y as u16))?;
        match self.dialogstack.last() {
            Some(d) => d.view(stdout),
            _ => self.page.view(stdout),
        }
    }
    pub fn resize(&mut self, bounds: &Bounds) {
        self.bounds = bounds.clone();
        self.page.resize(bounds.clone());
        for d in self.dialogstack.iter_mut() {
            d.resize(&bounds);
        }
    }
    pub fn update(&mut self, keycode: KeyCode) -> Option<TabMsg> {
        if let Some(d) = self.dialogstack.last_mut() {
            match d.update(keycode) {
                Some(DialogMsg::Submit) => {
                    let msg = match (&d.action, &d.input) {
                        (Action::Go(p), InputType::Choose((c, _))) => {
                            match c {
                                'y' => Some(TabMsg::Msg(ViewMsg::Go(p.clone()))),
                                _ => Some(TabMsg::Msg(ViewMsg::None)),
                            }
                        }
                        (Action::GoTo, InputType::Input(v)) => {
                            Some(TabMsg::Msg(ViewMsg::Go(v.clone())))
                        }
                        (Action::DeleteMe, InputType::Choose((c, _))) => {
                            match c {
                                'y' => Some(TabMsg::DeleteMe),
                                _ => Some(TabMsg::Msg(ViewMsg::None)),
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
            KeyCode::Char('v') => {
                let dialog = Dialog::new(
                    Action::DeleteMe,
                    String::from("Delete current tab?"),
                    InputType::Choose(('n', vec![
                        ('y', String::from("yes")), 
                        ('n', String::from("no"))])),
                    &self.bounds);
                self.dialogstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('p') => {
                let dialog = Dialog::new(
                    Action::GoTo,
                    String::from("enter path: "),
                    InputType::Input(String::from("")),
                    &self.bounds);
                self.dialogstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('i') => {
                self.page.movecursordown();
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('o') => {
                self.page.movecursorup();
                Some(TabMsg::Msg(ViewMsg::None))
            }
            KeyCode::Char('e') => {
                Some(TabMsg::CycleLeft)
            }
            KeyCode::Char('n') => {
                Some(TabMsg::CycleRight)
            }
            KeyCode::Enter => {
                let dialog = match self.page.selectundercursor() {
                    Tag::Text => Dialog::new(
                        Action::None,
                        String::from("You've selected some text. "),
                        InputType::None,
                        &self.bounds),
                    Tag::Heading => Dialog::new(
                        Action::None,
                        String::from("You've selected a heading "),
                        InputType::None,
                        &self.bounds),
                    Tag::Link(l) => Dialog::new(
                        Action::Go(l.to_string()),
                        String::from(format!("go to {}?", l)),
                        InputType::Choose(('n', vec![
                            ('y', String::from("yes")), 
                            ('n', String::from("no"))])),
                        &self.bounds),
                };
                self.dialogstack.push(dialog);
                Some(TabMsg::Msg(ViewMsg::None))
            }
            _ => None,
        }
    }
}
