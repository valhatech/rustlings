// `Vec<T>` est générique sur le type `T`. Dans la plupart des cas, le compilateur est capable
// d'inférer `T`, par exemple après avoir ajouté au vecteur une valeur d'un type concret.
// Mais dans cet exercice, le compilateur a besoin d'un coup de pouce via une annotation de type.

fn main() {
    // TODO : Corrigez l'erreur de compilation en annotant le type du vecteur
    // `Vec<T>`. Choisissez pour `T` un type entier qui peut être créé à partir de
    // `u8` et `i8`.
    let mut numbers = Vec::new();

    // Ne modifiez pas les lignes ci-dessous.
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
