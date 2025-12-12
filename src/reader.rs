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
pub struct Reader<T> {
    source:    Vec<(    T, String)>,
    wrapped:   Vec<(usize, String)>,
    bounds:    Bounds,
    scroll:    usize,
    cursor:    usize,
    maxscroll: usize,
    maxcursor: usize,
} 
impl<T: Clone + GetColors> Reader<T> {
    pub fn new(source: Vec<(T, String)>, bounds: Bounds) -> Self {
        let source       = source.clone();
        let wrapped      = Self::wraplist(&source, bounds.dim.w);
        let textlength   = wrapped.len();
        let (maxscroll, maxcursor) = 
            match textlength < bounds.dim.h {
                true  => (0, textlength),
                false => (textlength - bounds.dim.h, bounds.dim.h),
            };
        return Self {
            source:    source,
            wrapped:   wrapped,
            maxcursor: maxcursor,
            maxscroll: maxscroll,
            cursor:    0,
            scroll:    0,
            bounds:    bounds,
        }
    }
    pub fn resize(&mut self, newbounds: Bounds) {
        self.wrapped = Self::wraplist(&self.source, newbounds.dim.w);
        let textlength   = self.wrapped.len();
        // bounds cannot be filled
        if textlength < newbounds.dim.h {
            self.maxcursor = textlength;
            self.maxscroll = 0;
            self.cursor    = (textlength - 1) / 2;
            self.scroll    = 0;
        } 
        // bounds can be filled
        else {
            self.maxcursor = newbounds.dim.h;
            self.maxscroll = textlength - newbounds.dim.h;
            self.cursor    = (newbounds.dim.h - 1) / 2;
            self.scroll    = min(self.scroll, self.maxscroll);
        }
        self.bounds = newbounds;
    }
    pub fn view(&mut self, mut stdout: &Stdout) -> io::Result<()> {
        for (textindex, (sourceindex, text)) in 
            self.wrapped[self.scroll..(self.scroll + self.maxcursor)]
                .iter()
                .enumerate() 
        {
            let screencol = self.bounds.loc.x;
            let screenrow = self.bounds.loc.y + textindex;

            stdout
                .queue(cursor::MoveTo(screencol as u16, screenrow as u16))?
                .queue(style::SetColors(self.select(*sourceindex).getcolors()))?
                .queue(style::Print(text.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor as u16))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn movecursordown(&mut self) {
        if self.cursor < self.maxcursor - 1 {
            self.cursor += 1;
        } 
        else if self.scroll < self.maxscroll {
            self.scroll += 1;
        }
    }
    pub fn movecursorup(&mut self) {
        if 0 < self.cursor {
            self.cursor -= 1;
        } 
        else if 0 < self.scroll {
            self.scroll -= 1;
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
