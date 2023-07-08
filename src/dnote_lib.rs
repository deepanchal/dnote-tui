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
    pub uuid: String,
    pub content: String,
}
