// Il faut définir un panier de fruits sous la forme d'une table de hachage. La clé
// représente le nom du fruit et la valeur le nombre de fruits de ce type dans
// le panier. Vous devez mettre au moins 3 types de fruits différents
// (p. ex. pomme, banane, mangue) dans le panier, et le nombre total
// de fruits doit être d'au moins 5.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO : Déclarez la table de hachage.
    // let mut basket =

    // Deux bananes sont déjà fournies pour vous :)
    basket.insert(String::from("banana"), 2);

    // TODO : Mettez d'autres fruits dans votre panier.

    basket
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
