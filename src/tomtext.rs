// pager/src/tomtext

const LINK_SYMBOL:    &str = ".l";
const HEADING_SYMBOL: &str = ".h";

#[derive(Clone, PartialEq, Debug)]
pub enum TomTextData {
    Heading,
    Text, 
    Link(String),
} 
#[derive(Clone, PartialEq, Debug)]
pub struct TomTextLine {
    pub data: TomTextData,
    pub text: String,
} 
impl TomTextLine {
    fn parse_line(line: &str) -> Result<Self, String> {
        if let Some((symbol, mut text)) = line.split_at_checked(2) {
            text = text.trim();
            if symbol == LINK_SYMBOL {
                match text.split_once(' ') {
                    Some((link, txt)) =>
                        return Ok(Self {
                            data: TomTextData::Link(link.to_string()),
                            text: txt.to_string(),
                        }),
                    None => 
                        return Ok(Self {
                            data: TomTextData::Link(text.to_string()),
                            text: text.to_string(),
                        }),
                }
            }
            if symbol == HEADING_SYMBOL {
                return Ok(Self {
                    data: TomTextData::Heading,
                    text: text.to_string(),
                })
            }
        }
        Ok(Self {
            data: TomTextData::Text,
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
