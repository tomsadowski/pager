// pager/src/textview

use std::io::{self, Write, Stdout};
use crossterm::{
    QueueableCommand, cursor, terminal,
    style::{self, Colors}};

#[derive(Clone, Debug)]
pub struct TextView {
    source_text:  Vec<(Colors, String)>,
    display_text: Vec<(usize , String)>,
    scroll:       u16,
    width:        u16,
    height:       u16,
    cursor_x:     u16, 
    cursor_y:     u16,
} 
impl TextView {
    pub fn new(source: Vec<(Colors, String)>, width: u16, height: u16) -> Self {
        let source_text = source.clone();
        let wrapped: Vec<(usize, String)> = 
            get_indexed_wrapped(
                    source_text.iter().map(|x| &x.1).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();
        return Self {
            source_text:  source_text,
            display_text: wrapped,
            width:        width,
            height:       height,
            cursor_x:     0,
            cursor_y:     0,
            scroll:       0,
        }
    }
    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()> {
        let start = usize::from(self.scroll);
        let end   = {
            let height = usize::from(self.scroll + self.height);
            let length = self.display_text.len();
            if height < length {
                height
            } else {
                length
            }
        };
        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        for (i, l) in self.display_text[start..end].iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(0, i as u16))?
                .queue(style::SetColors(self.source_text[l.0].0))?
                .queue(style::Print(l.1.as_str()))?;
        }
        stdout.queue(cursor::MoveTo(self.cursor_x, self.cursor_y))?;
        stdout.flush()?;
        Ok(())
    }
    pub fn get_source_index_under_cursor(&self) -> usize {
        self.display_text[usize::from(self.cursor_y)].0
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width  = width;
        self.height = height;
        self.display_text = 
            get_indexed_wrapped(
                    self.source_text.iter().map(|x| &x.1).collect(), 
                    usize::from(width))
                .iter()
                .map(|x| (x.0, x.1.to_string()))
                .collect();
        self.cursor_x = 0;
        self.cursor_y = height / 2;
    }
    pub fn move_cursor_down(&mut self) {
        let end = {
            let height = self.height;
            let length = self.display_text.len() as u16 - self.scroll;
            if height < length {
                height
            } else {
                length
            }
        };
        if self.cursor_y < end - 1 {
            self.cursor_y += 1;
        } else if 
            (self.scroll + self.height - 1) < 
            ((self.display_text.len() as u16) - 1) 
        {
            self.scroll += 1;
        }
    }
    pub fn move_cursor_up(&mut self) {
        if self.cursor_y > 0 {
            self.cursor_y -= 1;
        } else if self.scroll > 0 {
            self.scroll -= 1;
        }
    }
} 
fn get_indexed_wrapped<'a: 'b, 'b>(lines: Vec<&'a String>, width: usize) 
    -> Vec<(usize, &'b str)> 
{
    let mut wrapped: Vec<(usize, &str)> = vec![];
    for (i, l) in lines.iter().enumerate() {
        let v = get_wrapped(l, width);
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
