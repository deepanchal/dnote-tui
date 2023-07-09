use dnote_tui::dnote_lib::*;

fn main() -> Result<(), DnoteClientError> {
    let client = DnoteClient {};
    let books = client.view_books()?;
    println!("Found books {:?}", books);

    let book_name = &books[0].name;
    let pages = client.view_pages(book_name)?;
    println!("Found pages in Book::{:?}: {:?}", book_name, pages);

    let page_id = pages[0].id;
    let page_info = client.view_page_info(page_id)?;
    println!("Found page content with id {}:\n {:?}", page_id, page_info);

    Ok(())
}
