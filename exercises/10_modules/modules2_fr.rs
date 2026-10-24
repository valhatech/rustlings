// Vous pouvez importer des chemins de modules dans la portée et leur donner de
// nouveaux noms avec les mots-clés `use` et `as`.

mod delicious_snacks {
    // TODO : Ajoutez les deux instructions `use` suivantes après les avoir corrigées.
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE,
    );
}
