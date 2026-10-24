// Les durées de vie sont aussi nécessaires quand des structs contiennent des références.

// TODO : Corrigez les erreurs de compilation liées à la struct.
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
