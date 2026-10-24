// TODO : Corrigez l'erreur de compilation sans sortir la définition de la macro de ce
// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
