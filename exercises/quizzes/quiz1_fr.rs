// Ceci est un quiz sur les sections suivantes :
// - Variables
// - Fonctions
// - If
//
// Mary achète des pommes. Le prix d'une pomme est calculé ainsi :
// - Une pomme coûte 2 rustbucks.
// - Cependant, si Mary achète plus de 40 pommes, le prix de chaque pomme de
// toute la commande est réduit à seulement 1 rustbuck !

// TODO : Écrivez une fonction qui calcule le prix d'une commande de pommes en fonction
// de la quantité achetée.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

// Ne modifiez pas les tests !
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
