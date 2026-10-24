// À la compilation, Rust doit connaître la place qu'occupe un type. Cela
// devient problématique pour les types récursifs, où une valeur peut contenir
// elle-même une autre valeur du même type. Pour contourner le problème, on peut utiliser une
// `Box` — un pointeur intelligent qui stocke des données sur le tas, ce qui permet aussi
// d'encapsuler un type récursif.
//
// Le type récursif que nous implémentons dans cet exercice est la « cons list », une
// structure de données fréquente dans les langages de programmation fonctionnelle. Chaque
// élément d'une cons list contient deux éléments : la valeur de l'élément courant et
// l'élément suivant. Le dernier élément est une valeur appelée `Nil`.

// TODO : Utilisez une `Box` dans la définition de l'enum pour que le code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO : Créez une cons list vide.
fn create_empty_list() -> List {
    todo!()
}

// TODO : Créez une cons list non vide.
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
