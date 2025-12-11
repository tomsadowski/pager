// pager/src/tag

const LINK_SYMBOL:    &str = ".l";
const HEADING_SYMBOL: &str = ".h";

#[derive(Clone, PartialEq, Debug)]
pub enum TextTag {
    Heading,
    Text, 
    Link(String),
} 
#[derive(Clone, PartialEq, Debug)]
pub struct TaggedText {
    pub tag:  TextTag,
    pub text: String,
} 
impl TaggedText {
    fn parse_line(line: &str) -> Result<Self, String> {
        if let Some((symbol, mut text)) = line.split_at_checked(2) {
            text = text.trim();
            if symbol == LINK_SYMBOL {
                match text.split_once(' ') {
                    Some((link, txt)) =>
                        return Ok(Self {
                            tag:  TextTag::Link(link.to_string()),
                            text: txt.to_string(),
                        }),
                    None => 
                        return Ok(Self {
                            tag:  TextTag::Link(text.to_string()),
                            text: text.to_string(),
                        }),
                }
            }
            if symbol == HEADING_SYMBOL {
                return Ok(Self {
                    tag:  TextTag::Heading,
                    text: text.to_string(),
                })
            }
        }
        Ok(Self {
            tag:  TextTag::Text,
            text: line.to_string(),
        })
    }
    pub fn parse_doc(lines: Vec<&str>) -> Result<Vec<Self>, String> {
        let mut vec = vec![];
        // parse remaining lines
        for line in lines.iter() {
            let formatted = Self::parse_line(line).unwrap();
            vec.push(formatted);
        }
        Ok(vec)
    }
}
