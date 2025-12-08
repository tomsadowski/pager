// pager/src/util


pub fn get_indexed_wrapped<'a: 'b, 'b>(lines: Vec<&'a String>, width: usize) 
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

pub fn get_wrapped(line: &str, width: usize) -> Vec<&str> {
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
