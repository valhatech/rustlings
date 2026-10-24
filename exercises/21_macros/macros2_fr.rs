fn main() {
    my_macro!();
}

// TODO : Corrigez l'erreur de compilation en déplaçant toute la définition de cette macro.
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
