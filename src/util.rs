// pager/src/util

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
        if self.min <= (self.cur - step) {
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
        if usize::MIN <= (self.cur - step) {
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
