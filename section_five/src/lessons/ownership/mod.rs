

struct Book {
    pages: u32,
    rating: u32
}

fn borrow_display_page_count(book: &Book) -> () {
    println!("pages: {:?}", book.pages);
    return
}

fn borrow_display_rating(book: &Book) -> () {
    println!("rating: {:?}", book.rating);
    return
}

fn no_borrow_display_rating(book: Book) -> () {
    println!("rating: {:?}", book.rating);
    return
}

pub fn ownership() -> () {
    let book: Book = Book {pages: 5, rating: 9};
    borrow_display_page_count(&book);
    borrow_display_rating(&book);
    
    no_borrow_display_rating(book);
    // println!("{:?}", book.pages);
    return
}
