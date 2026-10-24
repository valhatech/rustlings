// Vous pouvez utiliser le mot-clé `use` pour importer dans votre portée des chemins de
// modules provenant de n'importe où, et surtout de la bibliothèque standard.

// TODO : Importez `SystemTime` et `UNIX_EPOCH` du module `std::time` dans
// votre portée. Points de style bonus si vous y arrivez en une seule ligne !
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
