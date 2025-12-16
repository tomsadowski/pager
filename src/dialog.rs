// pager/src/dialog

use crate::widget::{Bounds, Dimension, Position, Selector};
use crate::tag::{self, Tag};
use crossterm::{QueueableCommand, cursor, terminal, style};
use crossterm::event::{Event, KeyEvent, KeyEventKind, KeyCode};
use std::{env, fs};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum Action {
    FollowPath(String),
    Input(String),
    Acknowledge,
}
#[derive(Clone, Debug)]
pub enum DialogMsg {
    Stay,
    Back,
    Proceed(Action),
}
#[derive(Clone, Debug)]
pub struct Dialog {
    action: Action,
    msg:    String,
    size:   Dimension,
}
impl Dialog {
    pub fn default() -> Self {
        Self {
            action: Action::Acknowledge,
            msg:  String::from(""), 
            size: Dimension {w: 0, h: 0},
        }
    }
    pub fn new(action: Action, msg: String, size: Dimension) -> Self {
        Self {
            action: action,
            msg:    msg, 
            size:   size,
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 3))?
            .queue(style::Print(format!("{:?}", self.action)))?
            .queue(cursor::MoveTo(0, 5))?
            .queue(style::Print(self.msg.as_str()))?;
        Ok(())
    }
    pub fn resize(&mut self, dim: Dimension) {
        self.size = dim.clone();
    }
    pub fn update(&mut self, keycode: KeyCode) -> Option<DialogMsg> {
        match keycode {
            KeyCode::Enter  => {
                Some(DialogMsg::Proceed(self.action.clone()))
            }
            KeyCode::Backspace  => {
                match &self.action {
                    Action::Input(_) => {self.msg.pop();}
                    _ => {}
                }
                Some(DialogMsg::Stay)
            }
            KeyCode::Char(c)  => {
                match &self.action {
                    Action::Input(_) => self.msg.push(c),
                    _ => {}
                }
                Some(DialogMsg::Stay)
            }
            _ => None,
        }
    }
}
