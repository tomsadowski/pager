// pager/src/widget

use crate::tag::GetColors;
use crate::util::{self, Rect, ScrollingCursor};
use crossterm::{QueueableCommand, cursor, style};
use crossterm::event::{KeyCode};
use std::io::{self, Stdout};

#[derive(Clone, Debug)]
pub enum DialogMsg {
    None,
    Cancel,
    Submit,
}
#[derive(Clone, Debug)]
pub enum InputType {
    None,
    Choose((char, Vec<(char, String)>)),
    Input(String),
}
#[derive(Clone, Debug)]
pub struct Dialog<T> {
    pub action: T,
    pub input: InputType,
    rect: Rect,
    prompt: String,
}
impl<T: Clone + std::fmt::Debug> Dialog<T> {
    pub fn new(rect: &Rect, action: T, input: InputType, prompt: &str) -> Self {
        Self {
            rect: rect.clone(),
            action: action,
            input: input,
            prompt: String::from(prompt), 
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        stdout
            .queue(cursor::MoveTo(0, self.rect.y + 2))?
            .queue(style::Print(self.prompt.as_str()))?
            .queue(cursor::MoveTo(0, self.rect.y + 4))?
            .queue(style::Print(format!("{:?}", self.input)))?;
        Ok(())
    }
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = rect.clone();
    }
    pub fn update(&mut self, keycode: &KeyCode) -> Option<DialogMsg> {
        match (&mut self.input, keycode) {
            (InputType::Choose(_), KeyCode::Enter) => {
                Some(DialogMsg::None)
            }
            (_, KeyCode::Enter) => {
                Some(DialogMsg::Submit)
            }
            (_, KeyCode::Esc) => {
                Some(DialogMsg::Cancel)
            }
            (InputType::Input(v), KeyCode::Backspace) => {
                v.pop();
                Some(DialogMsg::None)
            }
            (InputType::Input(v), KeyCode::Char(c)) => {
                v.push(*c);
                Some(DialogMsg::None)
            }
            (InputType::Choose(t), KeyCode::Char(c)) => {
                let chars: Vec<char> = t.1.iter().map(|e| e.0).collect();
                match chars.contains(&c) {
                    true => {
                        t.0 = *c;
                        Some(DialogMsg::Submit)
                    }
                    false => None,

                }
            }
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct Selector<T> {
    wrap: bool,
    rect: Rect,
    pub cursor: ScrollingCursor,
    source: Vec<(T, String)>,
    display: Vec<(usize, String)>,
} 
impl<T: Clone + GetColors> Selector<T> {
    pub fn new(rect: &Rect, source: Vec<(T, String)>, wrap: bool) -> Self {
        let display = 
            match wrap {
                true => util::wraplist(&source, rect.w),
                false => util::cutlist(&source, rect.w),
            };
        let textlength = display.len();
        return Self {
            wrap: wrap,
            source: source,
            display: display,
            cursor: ScrollingCursor::new(textlength, &rect),
            rect: rect.clone(),
        }
    }
    pub fn resize(&mut self, rect: &Rect) {
        self.rect = rect.clone();
        self.display = match self.wrap {
            true => util::wraplist(&self.source, rect.w),
            false => util::cutlist(&self.source, rect.w),
        };
        self.cursor.resize(self.display.len(), rect);
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let (start, end) = self.cursor.slicebounds();
        for (textindex, (sourceindex, text)) in 
            self.display[start..end].iter().enumerate() 
        {
            let screenrow = self.rect.y + textindex as u16;
            stdout
                .queue(cursor::MoveTo(self.rect.x, screenrow))?
                .queue(style::SetColors(self.source[*sourceindex].0.getcolors()))?
                .queue(style::Print(text.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor.cursor))?;
        Ok(())
    }
    pub fn selectundercursor(&self) -> &T {
        &self.source[self.display[self.cursor.index()].0].0
    }
} 
