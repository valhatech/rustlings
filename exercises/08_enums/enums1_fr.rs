#[derive(Debug)]
enum Message {
    // TODO : Définissez quelques types de messages, comme utilisés ci-dessous.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
