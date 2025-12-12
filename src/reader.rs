// pager/src/reader

use crate::util::{GetColors, wrap};
use std::io::{self, Write, Stdout};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style};

#[derive(Clone, Debug)]
pub struct Reader<T> {
    source:   Vec<(    T, String)>,
    wrapped:  Vec<(usize, String)>,
    scroll:   usize,
    cursor:   usize,
    mxscroll: usize,
    maxlines: usize,
} 
impl<T: Clone + GetColors> Reader<T> {
    pub fn new(source: Vec<(T, String)>, width: usize, height: usize) -> Self 
    {
        let source       = source.clone();
        let wrapped      = Self::wraplist(&source, width);
        let textlength   = wrapped.len();
        let screenlength = usize::from(height);
        let (mxscroll, maxlines) = 
            match textlength < screenlength {
                true  => (0, textlength),
                false => (textlength - screenlength, screenlength),
            };
        return Self {
            source:   source,
            wrapped:  wrapped,
            maxlines: maxlines,
            mxscroll: mxscroll,
            cursor:   0,
            scroll:   0,
        }
    }
    pub fn resize(&mut self, width: usize, height: usize) {
        // length of display text determined by screen width
        self.wrapped     = Self::wraplist(&self.source, width);
        let textlength   = self.wrapped.len();
        let screenlength = height;
        // screen cannot be filled
        if textlength < screenlength {
            self.maxlines  = textlength;
            self.mxscroll = 0;
            self.cursor   = (textlength - 1) / 2;
            self.scroll   = 0;
        } 
        // screen can be filled
        else {
            self.maxlines  = screenlength;
            self.mxscroll = textlength - screenlength;
            self.cursor   = (screenlength - 1) / 2;
            self.scroll   = std::cmp::min(self.scroll, self.mxscroll);
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        // clear everything
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        // display text
        for (screenindex, wrappedpair) in 
            self.wrapped[self.scroll..(self.scroll + self.maxlines)]
                .iter()
                .enumerate() 
        {
            stdout
                .queue(cursor::MoveTo(0, screenindex as u16))?
                .queue(style::SetColors(self.select(screenindex).getcolors()))?
                .queue(style::Print(wrappedpair.1.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor as u16))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn mvcursordown(&mut self) {
        if self.cursor < self.maxlines - 1 {
            self.cursor += 1;
        } 
        else if self.scroll < self.mxscroll {
            self.scroll += 1;
        }
    }
    pub fn mvcursorup(&mut self) {
        if 0 < self.cursor {
            self.cursor -= 1;
        } 
        else if 0 < self.scroll {
            self.scroll -= 1;
        }
    }
    pub fn selectundercursor(&self) -> &T {
        &self.select(self.cursor)
    }
    pub fn select(&self, i: usize) -> &T {
        &self.source[self.wrapped[i].0].0
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
