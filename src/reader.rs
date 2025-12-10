// pager/src/reader

use std::io::{self, Write, Stdout};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style::{self, Colors}};

#[derive(Clone, Debug)]
pub struct Reader<T> {
    source:   Vec<(T, Colors, String)>,
    slices:   Vec<(usize, String)>,
    scroll:   usize,
    cursor:   usize,
    mxscroll: usize,
    mxlines:  usize,
} 
impl<T: Clone> Reader<T> {
    pub fn new(source: Vec<(T, Colors, String)>, width: u16, height: u16) -> Self 
    {
        let source = source.clone();
        let slices: Vec<(usize, String)> = 
            Self::getindexedwrapped(
                    source.iter().map(|x| &x.2).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();

        let textlength   = slices.len();
        let screenlength = usize::from(height);
        let (mxscroll, mxlines) = 
            match textlength < screenlength {
                true  => (0, textlength),
                false => (textlength - screenlength, screenlength),
            };

        return Self {
            source:   source,
            slices: slices,
            mxlines:      mxlines,
            mxscroll:     mxscroll,
            cursor:       0,
            scroll:       0,
        }
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        // length of display text determined by screen width
        self.slices = 
            Self::getindexedwrapped(
                    self.source.iter().map(|x| &x.2).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();
        let textlength   = self.slices.len();
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
            self.slices[self.scroll..(self.scroll + self.mxlines)]
                .iter()
                .enumerate() 
        {
            stdout
                .queue(cursor::MoveTo(0, screenindex as u16))?
                .queue(style::SetColors(self.source[wrappedpair.0].1))?
                .queue(style::Print(wrappedpair.1.as_str()))?;
        }

        stdout.queue(cursor::MoveTo(0, self.cursor as u16))?;
        stdout.flush()?;
        Ok(())
    }
    // the index returned should be a key to the original (unwrapped) data
    pub fn select(&self) -> T {
        self.source[self.slices[self.cursor].0].0.clone()
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
    pub fn getindexedwrapped<'a: 'b, 'b>(lines: Vec<&'a String>, width: usize) 
        -> Vec<(usize, &'b str)> 
    {
        let mut wrapped: Vec<(usize, &str)> = vec![];
        for (i, l) in lines.iter().enumerate() {
            let v = Self::getwrapped(l, width);
            for s in v.iter() {
                wrapped.push((i, s));
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
