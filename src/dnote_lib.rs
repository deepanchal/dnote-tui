use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct DnoteBook {
    pub name: String,
}

impl FromStr for DnoteBook {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = s.trim().to_string();
        Ok(DnoteBook { name })
    }
}

#[derive(Debug, Clone)]
pub struct DnotePage {
    pub id: u32,
}

impl FromStr for DnotePage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s
            .trim()
            .trim_start_matches('(')
            .split(')')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        Ok(DnotePage { id })
    }
}

#[derive(Debug, Clone)]
pub struct DnotePageInfo {
    pub content: String,
}

impl FromStr for DnotePageInfo {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let content = s.trim().to_string();
        Ok(DnotePageInfo { content })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_dnotebook_from_string() {
        let s = String::from("my notebook");
        let book: DnoteBook = s.parse().unwrap();
        assert_eq!(book.name, "my notebook")
    }

    #[test]
    fn should_parse_dnotepage_from_string() {
        let input1 = "(21) # Issues [---More---]";
        let input2 = "  (27) # Missed [---More---]";
        let page1: DnotePage = input1.parse().unwrap();
        let page2: DnotePage = input2.parse().unwrap();
        assert_eq!(page1.id, 21);
        assert_eq!(page2.id, 27);
    }
}
