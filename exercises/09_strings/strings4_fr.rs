// Les appels à cette fonction doivent être remplacés par des appels à `string_slice` ou `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO : Voici un ensemble de valeurs — certaines sont des `String`, d'autres des `&str`.
// Votre tâche est de remplacer `placeholder(…)` par `string_slice(…)`
// ou `string(…)` selon ce que vous pensez être chaque valeur.
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder(format!("Interpolation {}", "Station"));

    // ATTENTION : Il s'agit d'une indexation par octets, pas par caractères.
    // L'indexation par caractères se fait avec `s.chars().nth(INDEX)`.
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
