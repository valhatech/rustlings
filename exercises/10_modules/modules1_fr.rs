// TODO : Corrigez l'erreur de compilation liée à l'appel d'une fonction privée.
mod sausage_factory {
    // Ne laissez personne en dehors de ce module voir ceci !
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
