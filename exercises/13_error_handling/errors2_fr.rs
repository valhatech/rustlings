// Supposons que nous écrivions un jeu où l'on peut acheter des objets avec des jetons. Tous les objets coûtent
// 5 jetons, et chaque achat d'objets entraîne des frais de traitement de 1
// jeton. Un joueur saisit le nombre d'objets qu'il veut acheter, et
// la fonction `total_cost` calcule le coût total des objets. Comme
// le joueur a saisi la quantité, nous la recevons sous forme de chaîne. Il a pu
// taper n'importe quoi, pas seulement des nombres !
//
// Pour l'instant, cette fonction ne gère pas du tout le cas d'erreur. Ce que nous voulons
// faire : si nous appelons la fonction `total_cost` avec une chaîne qui n'est pas un
// nombre, cette fonction renverra une `ParseIntError`. Dans ce cas, nous voulons
// renvoyer immédiatement cette erreur depuis notre fonction, sans essayer de multiplier et
// d'additionner.
//
// Il existe au moins deux façons de l'implémenter, toutes deux correctes. Mais l'une
// est bien plus courte !

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO : Gérez le cas d'erreur comme décrit ci-dessus.
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
