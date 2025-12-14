// pager/src/selector

use crate::interface::GetColors;
use crate::util::{Cursor, Scroll, Bounds, wrap};
use crossterm::{QueueableCommand, cursor, terminal, style};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct Selector<T> {
    bounds:  Bounds,
    cursor:  Cursor,
    scroll:  Scroll,
    source:  Vec<(    T, String)>,
    display: Vec<(usize, String)>,
} 
impl<T: Clone + GetColors> Selector<T> {
    pub fn new(source: Vec<(T, String)>, bounds: Bounds) -> Self {
        let source     = source.clone();
        let display    = Self::wraplist(&source, bounds.dim.w);
        let textlength = display.len();
        let cursor     = Cursor::top(textlength, &bounds);
        let scroll     = Scroll::new(textlength, cursor.range());
        return Self {
            source:  source,
            display: display,
            cursor:  cursor,
            scroll:  scroll,
            bounds:  bounds,
        }
    }
    pub fn resize(&mut self, newbounds: Bounds) {
        self.display   = Self::wraplist(&self.source, newbounds.dim.w);
        let textlength = self.display.len();
        self.cursor    = Cursor::center(textlength, &newbounds);
        self.bounds    = newbounds;
        self.scroll.resize(textlength, self.cursor.range());
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let screencol = self.bounds.pos.x as u16;
        for (textindex, (sourceindex, text)) in 
            self.display[self.scroll.cur..(self.scroll.cur + self.cursor.max)]
                .iter()
                .enumerate() 
        {
            let screenrow = (self.bounds.pos.y + textindex) as u16;
            stdout
                .queue(cursor::MoveTo(screencol, screenrow))?
                .queue(style::SetColors(self.select(*sourceindex).getcolors()))?
                .queue(style::Print(text.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor.cur as u16))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn movecursordown(&mut self) {
        if !self.cursor.movedown(1) {
            self.scroll.movedown(1);
        }
    }
    pub fn movecursorup(&mut self) {
        if !self.cursor.moveup(1) {
            self.scroll.moveup(1);
        }
    }
    pub fn select(&self, i: usize) -> &T {
        &self.source[i].0
    }
    pub fn wraplist(lines: &Vec<(T, String)>, width: usize) 
        -> Vec<(usize, String)>
    {
        let mut display: Vec<(usize, String)> = vec![];
        for (i, (t, l)) in lines.iter().enumerate() {
            let v = wrap(l, width);
            for s in v.iter() {
                display.push((i, s.to_string()));
            }
        }
        display
    }
} 
