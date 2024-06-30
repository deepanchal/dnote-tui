use color_eyre::eyre::{eyre, Result};
use std::{process::Command, str::FromStr};

type NoteId = u32;

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnoteBook {
    pub name: String,
}

impl FromStr for DnoteBook {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self> {
        let name = s.trim().to_string();
        Ok(DnoteBook { name })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct DnotePage {
    pub id: NoteId,
    pub summary: String,
}

impl FromStr for DnotePage {
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split(')').collect();
        let id = parts[0]
            .trim()
            .trim_start_matches('(')
            .parse()
            .map_err(|_| eyre!("Parsing error"))?;
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
    type Err = color_eyre::Report;
    fn from_str(s: &str) -> Result<Self> {
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

impl DnoteClient {
    fn execute_command(&self, command: DnoteCommand) -> Result<String> {
        let (cmd, args) = match command {
            DnoteCommand::Add { book_name, note } => {
                ("add", vec![book_name, "-c".to_string(), note])
            }
            DnoteCommand::ViewBooks => ("view", vec!["--name-only".to_string()]),
            DnoteCommand::ViewByBook { book_name } => ("view", vec![book_name]),
            DnoteCommand::ViewByNoteId { note_id } => (
                "view",
                vec![note_id.to_string(), "--content-only".to_string()],
            ),
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
            DnoteCommand::RemoveBook { book_name } => ("rm", vec![book_name]),
            DnoteCommand::RemoveNoteById { note_id } => ("rm", vec![note_id.to_string()]),
        };
        let output = Command::new("dnote").arg(cmd).args(args).output()?;
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    }

    pub fn get_books(&self) -> Result<Vec<DnoteBook>> {
        let output = self.execute_command(DnoteCommand::ViewBooks)?;
        output.lines().map(|l| l.parse()).collect()
    }

    pub fn rename_book(&self, book_name: &str, new_book_name: &str) -> Result<()> {
        self.execute_command(DnoteCommand::EditBook {
            book_name: book_name.to_string(),
            new_name: Some(new_book_name.to_string()),
        })?;
        Ok(())
    }

    pub fn get_pages(&self, book_name: &str) -> Result<Vec<DnotePage>> {
        let output = self.execute_command(DnoteCommand::ViewByBook {
            book_name: book_name.to_string(),
        })?;
        output
            .lines()
            .skip(1) // skip first line e.g '  • on book ccu'
            .map(|l| l.parse())
            .collect()
    }

    pub fn get_page_content(&self, page_id: NoteId) -> Result<DnotePageInfo> {
        let output = self.execute_command(DnoteCommand::ViewByNoteId { note_id: page_id })?;
        output.parse()
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
