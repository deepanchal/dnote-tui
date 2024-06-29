use dnote_tui::dnote::*;

fn main() -> Result<(), DnoteClientError> {
    let client = DnoteClient {};
    let books = client.get_books()?;
    println!("Found books {:?}", books);

    let book_name = &books[0].name;
    let pages = client.get_pages(book_name)?;
    println!("Found pages in Book::{:?}: {:?}", book_name, pages);

    let page_id = pages[0].id;
    let page_info = client.get_page_content(page_id)?;
    println!("Found page content with id {}:\n {:?}", page_id, page_info);

    Ok(())
}
