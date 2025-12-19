// pager/src/util

// ideally there are no imports here.
// structs, enums, functions, and constants that are generally useful
// or fundamental to the rest of the program.

#[derive(Clone, Debug)]
pub enum View {
    Tab,
    History,
    Bookmarks,
    Quit,
}
#[derive(Clone, Debug)]
pub enum ViewMsg {
    None,
    Go(String),
    Switch(View),
}
// a rectangle specified by a point and some lengths
#[derive(Clone, Debug)]
pub struct Rect {
    pub x: u16, pub y: u16, pub w: u16, pub h: u16,
}
impl Rect {
    pub fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self {x: x, y: y, w: w, h: h}
    }
}
#[derive(Clone, Debug)]
pub struct ScrollingCursor {
    pub scroll: usize,
    pub maxscroll: usize,
    pub cursor: u16,
    pub rect: Rect,
}
impl ScrollingCursor {
    pub fn new(textlength: usize, rect: &Rect) -> Self {
        let len = match u16::try_from(textlength) {
            Ok(t) => t, _ => u16::MAX,
        };
        match len < rect.h {
            // no scrolling allowed
            true => Self {
                rect: Rect::new(rect.x, rect.y, rect.w, len),
                cursor: rect.y, 
                scroll: 0, 
                maxscroll: 0,
            },
            // scrolling allowed
            false => Self {
                rect: rect.clone(),
                cursor: rect.y, 
                scroll: 0, 
                maxscroll: textlength - usize::from(rect.h),
            },
        }
    }
    pub fn resize(&mut self, textlength: usize, rect: &Rect) {
        let len = match u16::try_from(textlength) {
            Ok(t) => t, _ => u16::MAX,
        };
        match len < rect.h {
            // no scrolling allowed
            true => {
                self.rect = Rect::new(rect.x, rect.y, rect.w, len);
                self.scroll = 0;
                self.maxscroll = 0;
            },
            // scrolling allowed
            false => {
                self.rect = rect.clone();
                self.scroll = std::cmp::min(self.scroll, self.maxscroll);
                self.maxscroll = textlength - usize::from(rect.h);
            },
        }
        self.cursor = (self.rect.y + self.rect.h - 1) / 2;
    }
    // scroll up when cursor is at highest position
    pub fn moveup(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if (self.rect.y + step) <= self.cursor {
            self.cursor -= step;
            true
        } else if usize::MIN + scrollstep <= self.scroll {
            self.scroll -= scrollstep;
            true
        } else {
            false
        }
    }
    // scroll down when cursor is at lowest position
    pub fn movedown(&mut self, step: u16) -> bool {
        let scrollstep = usize::from(step);
        if (self.cursor + step) <= (self.rect.y + self.rect.h - 1) {
            self.cursor += step;
            true 
        } else if (self.scroll + scrollstep) <= self.maxscroll {
            self.scroll += scrollstep;
            true
        } else {
            false
        }
    }
    pub fn slicebounds(&self) -> (usize, usize) {
        let a = self.scroll;
        let b = self.scroll + usize::from(self.rect.h);
        (a, b)
    }
    pub fn index(&self) -> usize {
        usize::from(self.cursor - self.rect.y)
    }
}
pub fn wrap(line: &str, screenwidth: u16) -> Vec<String> {
    let width = usize::from(screenwidth);
    let mut wrapped: Vec<String> = vec![];
    let mut start = 0;
    let mut end = width;
    let length = line.len();
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
                end = start + width;
            }
            None => {
                wrapped.push(String::from(longest));
                start = end;
                end += width;
            }
        }
    }
    if start < length {
        wrapped.push(String::from(&line[start..length]));
    }
    wrapped
}
pub fn cut(line: &str, screenwidth: u16) -> String {
    let mut width = usize::from(screenwidth);
    if line.len() < width {
        return String::from(line)
    } else {
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
pub fn cutlist<T>(lines: &Vec<(T, String)>, w: u16) -> Vec<(usize, String)> {
    let mut display: Vec<(usize, String)> = vec![];
    for (i, (t, l)) in lines.iter().enumerate() {
        display.push((i, cut(l, w)));
    }
    display
}
pub fn wraplist<T>(lines: &Vec<(T, String)>, w: u16) -> Vec<(usize, String)> {
    let mut display: Vec<(usize, String)> = vec![];
    for (i, (t, l)) in lines.iter().enumerate() {
        let v = wrap(l, w);
        for s in v.iter() {
            display.push((i, s.to_string()));
        }
    }
    display
}
