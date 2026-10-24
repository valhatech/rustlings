// Caractères (`char`)

fn main() {
    // Remarquez les apostrophes _simples_, qui diffèrent des guillemets doubles
    // que vous avez vus jusqu'ici.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO : Comme dans l'exemple précédent, déclarez ci-dessous une variable nommée `your_character`
    // avec votre caractère préféré.
    // Essayez une lettre, un chiffre (entre apostrophes), un caractère spécial, un caractère
    // d'une autre langue que la vôtre, un emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
