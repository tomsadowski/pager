// pager/src/reader

use std::io::{self, Write, Stdout};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style::{self, Colors}};

pub trait GetColors {
    fn getcolors(&self) -> Colors;
}

#[derive(Clone, Debug)]
pub struct Reader<T> {
    source:   Vec<(    T, String)>,
    wrapped:  Vec<(usize, String)>,
    scroll:   usize,
    cursor:   usize,
    mxscroll: usize,
    mxlines:  usize,
} 

impl<T: Clone + GetColors> Reader<T> {
    pub fn new(source: Vec<(T, String)>, width: usize, height: usize) -> Self 
    {
        let source  = source.clone();
        let wrapped = Self::getindexedwrapped(&source, width);

        let textlength   = wrapped.len();
        let screenlength = usize::from(height);
        let (mxscroll, mxlines) = 
            match textlength < screenlength {
                true  => (0, textlength),
                false => (textlength - screenlength, screenlength),
            };

        return Self {
            source:   source,
            wrapped:  wrapped,
            mxlines:  mxlines,
            mxscroll: mxscroll,
            cursor:   0,
            scroll:   0,
        }
    }
    pub fn resize(&mut self, width: usize, height: usize) {
        // length of display text determined by screen width
        self.wrapped = Self::getindexedwrapped(&self.source, width);
        let textlength   = self.wrapped.len();
        let screenlength = usize::from(height);

        // screen cannot be filled
        if textlength < screenlength {
            self.mxlines  = textlength;
            self.mxscroll = 0;
            self.cursor   = (textlength - 1) / 2;
            self.scroll   = 0;
        } 
        // screen can be filled
        else {
            self.mxlines  = screenlength;
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
            self.wrapped[self.scroll..(self.scroll + self.mxlines)]
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
    pub fn selectundercursor(&self) -> &T {
        &self.source[self.wrapped[self.cursor].0].0
    }
    pub fn select(&self, i: usize) -> &T {
        &self.source[self.wrapped[i].0].0
    }
    pub fn mvcursordown(&mut self) {
        if self.cursor < self.mxlines - 1 {
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
    pub fn getindexedwrapped(lines: &Vec<(T, String)>, width: usize) 
        -> Vec<(usize, String)>
    {
        let mut wrapped: Vec<(usize, String)> = vec![];
        for (i, (t, l)) in lines.iter().enumerate() {
            let v = Self::getwrapped(l, width);
            for s in v.iter() {
                wrapped.push((i, String::from(*s)));
            }
        }
        wrapped
    }
    pub fn getwrapped(line: &str, width: usize) -> Vec<&str> {
        let mut wrapped: Vec<&str> = vec![];
        let mut start  = 0;
        let mut end    = width;
        let     length = line.len();
        while end < length {
            let longest = &line[start..end];
            match longest.rsplit_once(' ') {
                Some((a, b)) => {
                    let shortest = match a.len() {
                        0 => b,
                        _ => a,
                    };
                    wrapped.push(shortest);
                    start += shortest.len();
                    end    = start + width;
                }
                None => {
                    wrapped.push(longest);
                    start = end;
                    end  += width;
                }
            }
        }
        if start < length {
            wrapped.push(&line[start..length]);
        }
        wrapped
    }
} 
