// TODO : Corrigez l'erreur de compilation de cette fonction.
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

// TODO : Lisez les tests pour comprendre le comportement attendu.
// Faites passer tous les tests sans les modifier.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // Cela signifie qu'appeler `picky_eater` avec l'argument "strawberry" doit renvoyer "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
