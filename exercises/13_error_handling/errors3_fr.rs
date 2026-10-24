// Ce programme essaie d'utiliser une version complète de la fonction
// `total_cost` de l'exercice précédent. Mais il ne fonctionne pas !
// Pourquoi ? Que faut-il faire pour le corriger ?

use std::num::ParseIntError;

// Ne modifiez pas cette fonction.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO : Corrigez l'erreur de compilation en modifiant la signature et le corps de la
// fonction `main`.
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Ne modifiez pas cette ligne.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}
