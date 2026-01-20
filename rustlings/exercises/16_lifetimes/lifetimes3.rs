// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
#[derive(Debug)]
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
    println!("{:?}", book);
    println!("{:?}", book);
}
