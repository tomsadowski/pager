// model

use crate::{
    msg::Message
};
use crossterm::{
    QueueableCommand, style, cursor, terminal
};
use std::io::{
    self, Write, Stdout
};

const LEFT:  char = 'e';
const DOWN:  char = 'i';
const UP:    char = 'o';
const RIGHT: char = 'n';
const QUIT:  char = 'q';


#[derive(Clone, Debug)]
pub struct Model<'a, 'b>
{
    quit:         bool,
    source_text:  Vec<&'a str>,
    display_text: Vec<&'b str>,
    scroll:       u16,
    size:         (u16, u16),
    cursor:       (u16, u16),
} 

impl<'a: 'b, 'b> Model<'a, 'b>
{
    pub fn init(text: &'a str, size: (u16, u16)) -> Self 
    {
        let source: Vec<&str> = text.lines().collect();
        let wrapped = Self::get_wrapped(&source, usize::from(size.0));

        return Self {
            quit:         false,
            source_text:  source,
            display_text: wrapped,
            size:         size,
            cursor:       (0, 0),
            scroll:       0,
        }
    }

    pub fn view(&self, mut stdout: &Stdout) -> io::Result<()>
    {
        let start = usize::from(self.scroll);
        let end   = usize::from(self.scroll + self.size.1);

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;

        for (i, l) in self.display_text[start..end].iter().enumerate() {
            stdout
                .queue(cursor::MoveTo(0, i as u16))?
                .queue(style::Print(l))?;
        }

        stdout.queue(cursor::MoveTo(self.cursor.0, self.cursor.1))?;

        stdout.flush()?;

        Ok(())
    }

    pub fn update(&mut self, msg: Message)
    {
        match msg 
        {
            Message::Resize(x, y) => self.resize((x, y)),

            Message::Code(c) => {
                match c {
                    UP   => self.move_cursor_up(),
                    DOWN => self.move_cursor_down(),
                    QUIT => self.quit = true,
                    _ => {}
                }
            }

            _ => {}
        }
    }

    pub fn quit(&self) -> bool 
    {
        self.quit
    }

    fn get_wrapped(lines: &Vec<&'a str>, width: usize) -> Vec<&'b str>
    {
        let mut wrapped: Vec<&str> = vec![];

        for l in lines.iter() {
            let mut start  = 0;
            let mut end    = width;
            let     length = l.len();

            while end < length {
                wrapped.push(&l[start..end]);
                start = end;
                end += width;
            }
            if start < length {
                wrapped.push(&l[start..length]);
            }
        }

        wrapped
    }

    fn resize(&mut self, size: (u16, u16)) 
    {
        self.size = size;
        self.display_text = 
            Self::get_wrapped(&self.source_text, usize::from(self.size.0));
    }

    fn move_cursor_down(&mut self)
    {
        if self.cursor.1 < self.size.1 - 1 
        {
            self.cursor.1 += 1;
        }
        else if 
            (self.scroll + self.size.1 - 1) < 
            ((self.display_text.len() as u16) - 1) 
        {
            self.scroll += 1;
        }
    }

    fn move_cursor_up(&mut self) 
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
