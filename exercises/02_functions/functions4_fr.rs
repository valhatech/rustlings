// Ce magasin fait des soldes : si le prix est un nombre pair, vous obtenez
// 10 Rustbucks de réduction, mais s'il est impair, la réduction est de 3 Rustbucks.
// Ne vous souciez pas des corps des fonctions, seules les signatures nous
// intéressent pour le moment.

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO : Corrigez la signature de la fonction.
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
