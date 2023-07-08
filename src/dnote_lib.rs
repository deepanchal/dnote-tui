use std::{process::Command, str::FromStr};

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

#[derive(Debug)]
pub struct DnoteClient {}

#[derive(Debug)]
pub enum DnoteClientError {
    InvalidCommand,
    ParseError,
    UnknownError,
}

impl DnoteClient {
    pub fn view_books(&self) -> Result<Vec<DnoteBook>, DnoteClientError> {
        println!("Viewing all books...");
        let output = Command::new("dnote")
            .arg("view")
            .arg("--name-only")
            .output();
        match output {
            Ok(v) => {
                let stdout = String::from_utf8(v.stdout);
                match stdout {
                    Ok(s) => {
                        let result: Result<Vec<DnoteBook>, _> =
                            s.lines().map(|l| l.parse()).collect();
                        match result {
                            Ok(v) => Ok(v),
                            Err(e) => Err(DnoteClientError::ParseError)
                        }
                    }
                    Err(e) => Err(DnoteClientError::UnknownError),
                }
            }
            Err(e) => Err(DnoteClientError::UnknownError),
        }
    }
    pub fn view_pages(&self, book_name: &str) {
        // Command: dnote view my_cool_book
        // Implementation:
        println!("Viewing pages for book: {}", book_name);
        // Your implementation here
    }
    pub fn view_page_info(&self, book_name: &str, page_id: u32) {
        // Command: dnote view my_cool_book 10 --content-only
        // Implementation:
        println!(
            "Viewing content for page {} in book: {}",
            page_id, book_name
        );
        // Your implementation here
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

    #[test]
    fn should_parse_dnotepageinfo_from_string() {
        let input1 = "# E2E\n\n- Grab a list of all data test ids on a page\n- Make sure all those data test ids exist";
        let input2 = "   # E2E   \n\n   - Grab a list of all data test ids on a page   \n   - Make sure all those data test ids exist   ";
        let input3 = "";
        let page_info1: DnotePageInfo = input1.parse().unwrap();
        let page_info2: DnotePageInfo = input2.parse().unwrap();
        let page_info3: DnotePageInfo = input3.parse().unwrap();
        assert_eq!(page_info1.content, input1);
        assert_eq!(page_info2.content, input2.trim());
        assert_eq!(page_info3.content, input3);
    }
}
