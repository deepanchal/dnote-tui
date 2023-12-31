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
    /// Truncated content from page
    pub summary: String,
}

impl FromStr for DnotePage {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(')').collect();
        let id = parts[0].trim().trim_start_matches('(').parse().unwrap();
        let summary = parts[1]
            .trim()
            .trim_end_matches("[---More---]")
            .trim()
            .to_string();
        Ok(DnotePage { id, summary })
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
    DnoteCommand,
    UTF8ParseError,
    ParseError,
    UnknownError,
}

impl DnoteClient {
    pub fn get_books(&self) -> Result<Vec<DnoteBook>, DnoteClientError> {
        // println!("Viewing all books...");
        let output = Command::new("dnote")
            .arg("view")
            .arg("--name-only")
            .output()
            .map_err(|_| DnoteClientError::DnoteCommand)?;
        let stdout: String =
            String::from_utf8(output.stdout).map_err(|_| DnoteClientError::UTF8ParseError)?;
        let result: Result<Vec<DnoteBook>, _> = stdout.lines().map(|l| l.parse()).collect();
        result.map_err(|_| DnoteClientError::ParseError)
    }
    pub fn get_pages(&self, book_name: &str) -> Result<Vec<DnotePage>, DnoteClientError> {
        // println!("Viewing pages for book: {}", book_name);
        let output = Command::new("dnote")
            .arg("view")
            .arg(book_name)
            .output()
            .map_err(|_| DnoteClientError::DnoteCommand)?;
        let stdout =
            String::from_utf8(output.stdout).map_err(|_| DnoteClientError::UTF8ParseError)?;
        let result: Result<Vec<DnotePage>, _> = stdout
            .lines()
            // skip first line e.g '  • on book ccu'
            .skip(1)
            .map(|l| l.parse())
            .collect();
        result.map_err(|_| DnoteClientError::ParseError)
    }
    pub fn get_page_content(&self, page_id: u32) -> Result<DnotePageInfo, DnoteClientError> {
        // println!("Viewing content for page with id {}", page_id);
        let output = Command::new("dnote")
            .arg("view")
            .arg(page_id.to_string())
            .arg("--content-only")
            .output()
            .map_err(|_| DnoteClientError::DnoteCommand)?;
        let stdout =
            String::from_utf8(output.stdout).map_err(|_| DnoteClientError::UTF8ParseError)?;
        stdout.parse().map_err(|_| DnoteClientError::ParseError)
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
        assert_eq!(page1.summary, "# Issues");
        assert_eq!(page2.id, 27);
        assert_eq!(page2.summary, "# Missed");
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
