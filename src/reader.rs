// pager/src/reader

use crate::{
    util::GetColors,
    util::wrap,
    util::Bounds,
};
use std::{
    cmp::min,
    io::{self, Write, Stdout},
};
use crossterm::{
    QueueableCommand, cursor, terminal, style
};


#[derive(Clone, Debug)]
pub struct Cursor {
    cur: usize,
    min: usize,
    max: usize,
}
impl Cursor {
    pub fn top(textlength: usize, bounds: &Bounds) -> Self {
        match textlength < bounds.dim.h {
            true  => Self {
                cur: bounds.pos.y, 
                min: bounds.pos.y,
                max: bounds.pos.y + textlength,
            },
            false => Self {
                cur: bounds.pos.y, 
                min: bounds.pos.y,
                max: bounds.pos.y + bounds.dim.h,
            },
        }
    }
    pub fn center(textlength: usize, bounds: &Bounds) -> Self {
        match textlength < bounds.dim.h {
            true  => Self {
                cur: (textlength - 1) / 2,
                min: bounds.pos.y,
                max: bounds.pos.y + textlength,
            },
            false => Self {
                cur: (bounds.dim.h - 1) / 2,
                min: bounds.pos.y,
                max: bounds.pos.y + bounds.dim.h,
            },
        }
    }
    pub fn range(&self) -> usize {
        self.max - self.min
    }
    pub fn moveup(&mut self, step: usize) -> bool {
        if self.min <= (self.cur - step) {
            self.cur -= step;
            return true
        } 
        false
    }
    pub fn movedown(&mut self, step: usize) -> bool {
        if (self.cur + step) <= (self.max - 1) {
            self.cur += step;
            return true 
        }
        false
    }
}

#[derive(Clone, Debug)]
pub struct Scroll {
    cur: usize,
    max: usize,
}
impl Scroll {
    pub fn new(textlength: usize, cursor: &Cursor) -> Self {
        let range = cursor.range();
        match textlength <= range  {
            true  => Self {
                cur: 0, 
                max: 0,
            },
            false => Self {
                cur: 0, 
                max: textlength - range
            },
        }
    }
    pub fn resize(&mut self, textlength: usize, cursor: &Cursor) {
        let range = cursor.range();
        match textlength <= range  {
            true  => {
                self.cur = 0;
                self.max = 0;
            },
            false => {
                self.max = textlength - range;
                self.cur = min(self.cur, self.max);
            },
        }
    }
    pub fn moveup(&mut self, step: usize) -> bool {
        if usize::MIN <= (self.cur - step) {
            self.cur -= step;
            return true
        } 
        false
    }
    pub fn movedown(&mut self, step: usize) -> bool {
        if (self.cur + step) <= self.max {
            self.cur += step;
            return true
        } 
        false
    }
}

#[derive(Clone, Debug)]
pub struct Reader<T> {
    bounds:  Bounds,
    scroll:  Scroll,
    cursor:  Cursor,
    source:  Vec<(    T, String)>,
    wrapped: Vec<(usize, String)>,
} 
impl<T: Clone + GetColors> Reader<T> {
    pub fn new(source: Vec<(T, String)>, bounds: Bounds) -> Self {
        let source     = source.clone();
        let wrapped    = Self::wraplist(&source, bounds.dim.w);
        let textlength = wrapped.len();
        let cursor     = Cursor::top(textlength, &bounds);
        let scroll     = Scroll::new(textlength, &cursor);
        return Self {
            source:  source,
            wrapped: wrapped,
            cursor:  cursor,
            scroll:  scroll,
            bounds:  bounds,
        }
    }
    pub fn resize(&mut self, newbounds: Bounds) {
        self.wrapped   = Self::wraplist(&self.source, newbounds.dim.w);
        let textlength = self.wrapped.len();
        self.cursor    = Cursor::center(textlength, &newbounds);
        self.bounds    = newbounds;
        self.scroll.resize(textlength, &self.cursor);
    }
    pub fn view(&mut self, mut stdout: &Stdout) -> io::Result<()> {
        for (textindex, (sourceindex, text)) in 
            self.wrapped[self.scroll.cur..(self.scroll.cur + self.cursor.max)]
                .iter()
                .enumerate() 
        {
            let screencol = self.bounds.pos.x;
            let screenrow = self.bounds.pos.y + textindex;

            stdout
                .queue(cursor::MoveTo(screencol as u16, screenrow as u16))?
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
        let mut wrapped: Vec<(usize, String)> = vec![];
        for (i, (t, l)) in lines.iter().enumerate() {
            let v = wrap(l, width);
            for s in v.iter() {
                wrapped.push((i, s.to_string()));
            }
        }
        wrapped
    }
} 
