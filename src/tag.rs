// pager/src/tag

const LINK_SYMBOL:    &str = ".l";
const HEADING_SYMBOL: &str = ".h";

#[derive(Clone, PartialEq, Debug)]
pub enum Tag {
    Heading,
    Text, 
    Link(String),
} 
pub fn parse_doc(lines: Vec<&str>) -> Result<Vec<(Tag, String)>, String> {
    let mut vec = vec![];
    for line in lines.iter() {
        let formatted = parse_line(line).unwrap();
        vec.push(formatted);
    }
    Ok(vec)
}
pub fn parse_line(line: &str) -> Result<(Tag, String), String> {
    if let Some((symbol, mut text)) = line.split_at_checked(2) {
        text = text.trim();
        if symbol == LINK_SYMBOL {
            match text.split_once(' ') {
                Some((link, txt)) =>
                    return Ok((Tag::Link(link.to_string()), txt.to_string())),
                None => 
                    return Ok((Tag::Link(text.to_string()), text.to_string())),
            }
        }
        if symbol == HEADING_SYMBOL {
            return Ok((Tag::Heading, text.to_string()))
        }
    }
    Ok((Tag::Text, line.to_string()))
}
