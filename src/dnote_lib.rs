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

