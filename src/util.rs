// pager/src/util

use crossterm::style::Colors;

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

pub trait GetColors {
    fn getcolors(&self) -> Colors;
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
