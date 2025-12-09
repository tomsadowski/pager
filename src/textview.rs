// pager/src/textview

use std::io::{self, Write, Stdout};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style::{self, Colors}};

#[derive(Clone, Debug)]
pub struct TextView {
    source_text:  Vec<(Colors, String)>,
    display_text: Vec<(usize , String)>,
    scroll:       usize,
    cursor:       usize,
    max_scroll:   usize,
    max_lines:    usize,
} 
impl TextView {
    pub fn new(source: Vec<(Colors, String)>, width: u16, height: u16) -> Self {
        let source_text = source.clone();
        let wrapped: Vec<(usize, String)> = 
            Self::get_indexed_wrapped(
                    source_text.iter().map(|x| &x.1).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();
        let text_length = wrapped.len();
        let scrn_length = usize::from(height);
        let (max_scroll, max_lines) = 
            match text_length < scrn_length {
                true  => (0                        , text_length),
                false => (text_length - scrn_length, scrn_length),
            };

        return Self {
            source_text:  source_text,
            display_text: wrapped,
            max_lines:    max_lines,
            max_scroll:   max_scroll,
            cursor:       0,
            scroll:       0,
        }
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        // length of display text determined by screen width
        self.display_text = 
            Self::get_indexed_wrapped(
                    self.source_text.iter().map(|x| &x.1).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();
        let text_length = self.display_text.len();
        let scrn_length = usize::from(height);
        // screen cannot be filled, reset cursor and scroll to beginning
        if text_length < scrn_length {
            self.max_lines  = text_length;
            self.max_scroll = 0;
            self.cursor     = (text_length - 1) / 2;
            self.scroll     = 0;
        } 
        // screen can be filled
        else {
            self.max_lines  = scrn_length;
            self.max_scroll = text_length - scrn_length;
            self.cursor     = (scrn_length - 1) / 2;
            self.scroll     = std::cmp::min(self.scroll, self.max_scroll);
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        // clear everything
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        // display text
        for (i, l) in 
            self.display_text[self.scroll..(self.scroll + self.max_lines)]
                .iter()
                .enumerate() 
        {
            stdout
                .queue(cursor::MoveTo(0, i as u16))?
                .queue(style::SetColors(self.source_text[l.0].0))?
                .queue(style::Print(l.1.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(0, self.cursor as u16))?;
        stdout.flush()?;
        Ok(())
    }
    // the index returned should be a key to the original (unwrapped) data
    pub fn get_index_under_cursor(&self) -> usize {
        self.display_text[self.cursor].0
    }
    pub fn move_cursor_down(&mut self) {
        if self.cursor < self.max_lines - 1 {
            self.cursor += 1;
        } 
        else if self.scroll < self.max_scroll {
            self.scroll += 1;
        }
    }
    pub fn move_cursor_up(&mut self) {
        if 0 < self.cursor {
            self.cursor -= 1;
        } 
        else if 0 < self.scroll {
            self.scroll -= 1;
        }
    }
    fn get_indexed_wrapped<'a: 'b, 'b>(lines: Vec<&'a String>, width: usize) 
        -> Vec<(usize, &'b str)> 
    {
        let mut wrapped: Vec<(usize, &str)> = vec![];
        for (i, l) in lines.iter().enumerate() {
            let v = Self::get_wrapped(l, width);
            for s in v.iter() {
                wrapped.push((i, s));
            }
        }
        wrapped
    }
    fn get_wrapped(line: &str, width: usize) -> Vec<&str> {
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
