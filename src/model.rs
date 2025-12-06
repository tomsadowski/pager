// model

use crossterm::{
    QueueableCommand,
    style,
    cursor,
    terminal,
};
use std::io::{
    self, 
    Write,
    Stdout,
};

#[derive(Clone, Debug)]
pub struct Model<'a>
{
    pub quit: bool,

    lines:    Vec<&'a str>,
    scroll:   u16,
    size:    (u16, u16),
    cursor:  (u16, u16),
} 
impl<'a> Model<'a>
{
    pub fn init(text: &'a str, size: (u16, u16)) -> Self 
    {
        return Self 
        {
            quit:   false,

            lines:  text.lines().map(|l| l).collect(),

            size:   size,
            cursor: (0, 0),
            scroll:  0,
        }
    }

    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()>
    {
        let start = usize::from(self.scroll);
        let end   = usize::from(self.scroll + self.size.1);

        stdout
            .queue(terminal::Clear(terminal::ClearType::All))?;

        for (i, l) in self.lines[start..end].iter().enumerate() 
        {
            stdout
                .queue(cursor::MoveTo(0, i as u16))?
                .queue(style::Print(l))?;
        }

        stdout
            .queue(cursor::MoveTo(self.cursor.0, self.cursor.1))?;

        stdout.flush()?;

        Ok(())
    }

    pub fn resize(&mut self, size: (u16, u16)) 
    {
    }

    pub fn move_cursor_down(&mut self)
    {
        if self.cursor.1 < self.size.1 - 1 
        {
            self.cursor.1 += 1;
        }
        else if 
            (self.scroll + self.size.1 - 1) < 
            ((self.lines.len() as u16) - 1) 
        {
            self.scroll += 1;
        }
    }

    pub fn move_cursor_up(&mut self) 
    {
        if self.cursor.1 > 0 
        {
            self.cursor.1 -= 1;
        }
        else if self.scroll > 0 
        {
            self.scroll -= 1;
        }
    }
} 
