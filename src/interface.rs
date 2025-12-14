// pager/src/interface

use crossterm::style::{Color, Colors};
use crate::tag::Tag;

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
                Color::Rgb {r: 225,  g: 180,  b: 105},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
            Tag::Link(_) => Colors::new(
                Color::Rgb {r: 180,  g: 105,  b: 225},
                Color::Rgb {r:   0,  g:   0,  b:   0},
            ),
        } 
    }
}
