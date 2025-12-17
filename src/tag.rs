// pager/src/tag

use crossterm::style::{Color, Colors};

pub trait GetColors {
    fn getcolors(&self) -> Colors;
}
impl GetColors for Tag {
    fn getcolors(&self) -> Colors {
        match self {
            Tag::Heading => Colors::new(
                Color::Rgb {r: 225,  g: 105,  b:  180},
                Color::Rgb {r:   0,  g:   0,  b:    0},
            ),
            Tag::Text => Colors::new(
                Color::Rgb {r: 180,  g: 180,  b: 180},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
            Tag::Link(_) => Colors::new(
                Color::Rgb {r: 180,  g: 105,  b: 225},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
        } 
    }
}
#[derive(Clone, PartialEq, Debug)]
pub enum Tag {
    Heading,
    Text, 
    Link(String),
} 
pub fn parse_doc(lines: Vec<&str>) -> Vec<(Tag, String)> {
    let mut vec = vec![];
    for line in lines.iter() {
        let formatted = parse_line(line);
        vec.push(formatted);
    }
    vec
}
pub fn parse_line(line: &str) -> (Tag, String) {
    if let Some((symbol, mut text)) = line.split_at_checked(2) {
        text = text.trim();
        if symbol == ".l" {
            match text.split_once(' ') {
                Some((link, txt)) =>
                    return (Tag::Link(link.to_string()), txt.to_string()),
                None => 
                    return (Tag::Link(text.to_string()), text.to_string()),
            }
        }
        if symbol == ".h" {
            return (Tag::Heading, text.to_string())
        }
    }
    (Tag::Text, line.to_string())
}
