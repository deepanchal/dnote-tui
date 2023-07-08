#[derive(Debug, Clone)]
pub struct DnoteBook {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct DnotePage {
    pub id: u32,
    pub uuid: String,
    pub content: String,
}
