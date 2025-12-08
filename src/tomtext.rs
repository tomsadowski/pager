// pager/src/tomtext


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
    const LINK_SYMBOL:    &str = ".l";
    const HEADING_SYMBOL: &str = ".h";

    fn parse_line(line: &str) -> Result<Self, String> {
        if let Some((symbol, text)) = line.split_at_checked(2) {
            if symbol == Self::LINK_SYMBOL {
                let (url, text) = Scheme::from_str(text).unwrap();
                return Ok(
                    Self {
                        data: TomTextData::Link(url),
                        text: text,
                    })
            }
            if symbol == Self::HEADING_SYMBOL {
                return Ok(
                    Self {
                        data: TomTextData::Heading,
                        text: text.to_string(),
                    })
            }
        }
        return Ok(
            Self {
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
