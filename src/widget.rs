// pager/src/widget

use crate::tag::GetColors;
use crossterm::{QueueableCommand, cursor, terminal, style};
use std::io::{self, Write, Stdout};

#[derive(Clone, Debug)]
pub struct Bounds {
    pub pos: Position,
    pub dim: Dimension,
}
#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize, 
    pub y: usize, 
}
#[derive(Clone, Debug)]
pub struct Dimension {
    pub w: usize, 
    pub h: usize,
}
#[derive(Clone, Debug)]
pub struct Cursor {
    pub cur: usize,
    pub min: usize,
    pub max: usize,
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
    pub fn moveup(&mut self, step: usize) -> bool {
        if (self.min + step) <= self.cur {
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
    pub fn range(&self) -> usize {
        self.max - self.min
    }
}
#[derive(Clone, Debug)]
pub struct Scroll {
    pub cur: usize,
    pub max: usize,
}
impl Scroll {
    pub fn new(textlength: usize, range: usize) -> Self {
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
    pub fn resize(&mut self, textlength: usize, range: usize) {
        match textlength <= range  {
            true  => {
                self.cur = 0;
                self.max = 0;
            },
            false => {
                self.max = textlength - range;
                self.cur = std::cmp::min(self.cur, self.max);
            },
        }
    }
    pub fn moveup(&mut self, step: usize) -> bool {
        if usize::MIN + step <= self.cur {
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
pub fn wrap(line: &str, width: usize) -> Vec<String> {
    let mut wrapped: Vec<String> = vec![];
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
                wrapped.push(String::from(shortest));
                start += shortest.len();
                end    = start + width;
            }
            None => {
                wrapped.push(String::from(longest));
                start = end;
                end  += width;
            }
        }
    }
    if start < length {
        wrapped.push(String::from(&line[start..length]));
    }
    wrapped
}
pub fn cut(line: &str, mut width: usize) -> String {
    if line.len() < width {
        return String::from(line)
    }
    else {
        width -= 2;
        let longest = &line[..width];
        match longest.rsplit_once(' ') {
            Some((a, b)) => {
                let shortest = match a.len() {
                    0 => b,
                    _ => a,
                };
                return format!("{}..", shortest)
            }
            None => {
                return format!("{}..", longest)
            }
        }

    }
}
#[derive(Clone, Debug)]
pub struct Selector<T> {
    wrap:    bool,
    bounds:  Bounds,
    cursor:  Cursor,
    scroll:  Scroll,
    source:  Vec<(    T, String)>,
    display: Vec<(usize, String)>,
} 
impl<T: Clone + GetColors> Selector<T> {
    pub fn new(source: Vec<(T, String)>, wrap: bool, bounds: Bounds) -> Self {
        let source     = source.clone();
        let display    = 
            match wrap {
                true  => Self::wraplist(&source, bounds.dim.w),
                false => Self::cutlist(&source, bounds.dim.w),
            };
        let textlength = display.len();
        let cursor     = Cursor::top(textlength, &bounds);
        let scroll     = Scroll::new(textlength, cursor.range());
        return Self {
            wrap:    wrap,
            source:  source,
            display: display,
            cursor:  cursor,
            scroll:  scroll,
            bounds:  bounds,
        }
    }
    pub fn resize(&mut self, newbounds: Bounds) {
        self.display = 
            match self.wrap {
                true  => Self::wraplist(&self.source, newbounds.dim.w),
                false => Self::cutlist(&self.source, newbounds.dim.w),
            };
        let textlength = self.display.len();
        self.cursor    = Cursor::center(textlength, &newbounds);
        self.bounds    = newbounds;
        self.scroll.resize(textlength, self.cursor.range());
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let screencol = self.bounds.pos.x as u16;
        for (textindex, (sourceindex, text)) in 
            self.display[self.scroll.cur..(self.scroll.cur + self.cursor.range())]
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
    pub fn selectundercursor(&self) -> &T {
        &self.source[self.cursor.cur].0
    }
    pub fn select(&self, i: usize) -> &T {
        &self.source[i].0
    }
    pub fn cutlist(lines: &Vec<(T, String)>, width: usize) 
        -> Vec<(usize, String)>
    {
        let mut display: Vec<(usize, String)> = vec![];
        for (i, (t, l)) in lines.iter().enumerate() {
            display.push((i, cut(l, width)));
        }
        display
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
