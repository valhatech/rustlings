// Le compilateur Rust doit savoir comment vérifier que les références fournies sont
// valides, afin de pouvoir prévenir le programmeur si une référence risque de
// sortir de la portée avant d'être utilisée. Rappelez-vous : les références sont des emprunts et
// ne possèdent pas leurs données. Que se passe-t-il si leur propriétaire sort de la portée ?

// TODO : Corrigez l'erreur de compilation en mettant à jour la signature de la fonction.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
