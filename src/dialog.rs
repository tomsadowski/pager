// pager/src/dialog

use crate::widget::{Bounds};
use crossterm::{QueueableCommand, cursor, terminal, style};
use crossterm::event::{KeyCode};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum Action {
    None,
    FollowPath(String),
}
#[derive(Clone, Debug)]
pub enum DialogMsg {
    None,
    Cancel,
    Submit,
}
#[derive(Clone, Debug)]
pub enum InputType {
    Choose(Vec<(char, String)>),
    Input(Vec<char>),
    None,
}
#[derive(Clone, Debug)]
pub struct Dialog {
    size:   Bounds,
    action: Action,
    prompt: String,
    input:  InputType,
}
impl Dialog {
    pub fn new(action: Action, 
               prompt: String, 
               input:  InputType, 
               size:   Bounds) -> Self 
    {
        Self {
            action: action,
            prompt: prompt, 
            input:  input,
            size:   size,
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 2))?
            .queue(style::Print(format!("{:?}", self.action)))?
            .queue(cursor::MoveTo(0, 4))?
            .queue(style::Print(self.prompt.as_str()))?
            .queue(cursor::MoveTo(0, 6))?
            .queue(style::Print(format!("{:?}", self.input)))?;
        Ok(())
    }
    pub fn resize(&mut self, bounds: Bounds) {
        self.size = bounds.clone();
    }
    pub fn update(&mut self, keycode: KeyCode) -> Option<DialogMsg> {
        match (&mut self.input, keycode) {
            (_, KeyCode::Enter)  => {
                Some(DialogMsg::Submit)
            }
            (InputType::Input(v), KeyCode::Backspace) => {
                v.pop();
                Some(DialogMsg::None)
            }
            (InputType::Input(v), KeyCode::Char(c))  => {
                v.push(c);
                Some(DialogMsg::None)
            }
            _ => None,
        }
    }
}
