use std::{io::Error, process::Command, str::FromStr};

type NoteId = u32;

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
    pub id: NoteId,
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
pub enum DnoteCommand {
    Add {
        book_name: String,
        note: String,
    },
    ViewBooks,
    ViewByBook {
        book_name: String,
    },
    ViewByNoteId {
        note_id: NoteId,
    },
    EditNoteById {
        note_id: String,
        new_content: Option<String>,
        new_book: Option<String>,
    },
    EditBook {
        book_name: String,
        new_name: Option<String>,
    },
    RemoveBook {
        book_name: String,
    },
    RemoveNoteById {
        note_id: NoteId,
    },
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
    fn execute_command(&self, command: DnoteCommand) -> Result<String, DnoteClientError> {
        let (cmd, args) = match command {
            DnoteCommand::Add { book_name, note } => {
                let args = vec![book_name, "-c".to_string(), note];
                ("add", args)
            }
            DnoteCommand::ViewBooks {} => {
                let args = vec!["--name-only".to_string()];
                ("view", args)
            }
            DnoteCommand::ViewByBook { book_name } => {
                let args = vec![book_name];
                ("view", args)
            }
            DnoteCommand::EditNoteById {
                note_id,
                new_content,
                new_book,
            } => {
                let mut args = vec![note_id];
                if let Some(content) = new_content {
                    args.push("-c".to_string());
                    args.push(content);
                }
                if let Some(book) = new_book {
                    args.push("-b".to_string());
                    args.push(book);
                }
                ("edit", args)
            }
            DnoteCommand::EditBook {
                book_name,
                new_name,
            } => {
                let mut args = vec![book_name];
                if let Some(name) = new_name {
                    args.push("-n".to_string());
                    args.push(name);
                }
                ("edit", args)
            }
            _ => todo!(),
        };
        let output = Command::new("dnote")
            .arg(cmd)
            .args(args)
            .output()
            .map_err(|_| DnoteClientError::DnoteCommand)?;
        let stdout: String =
            String::from_utf8(output.stdout).map_err(|_| DnoteClientError::UTF8ParseError)?;
        Ok(stdout)
    }

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
    pub fn rename_book(
        &self,
        book_name: &str,
        new_book_name: &str,
    ) -> Result<(), DnoteClientError> {
        // dnote edit stm -n t3
        let output = Command::new("dnote")
            .arg("edit")
            .arg(book_name)
            .arg("-n")
            .arg(new_book_name)
            .output()
            .map_err(|_| DnoteClientError::DnoteCommand)?;
        let _stdout: String =
            String::from_utf8(output.stdout).map_err(|_| DnoteClientError::UTF8ParseError)?;
        Ok(())
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
    pub fn get_page_content(&self, page_id: NoteId) -> Result<DnotePageInfo, DnoteClientError> {
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
