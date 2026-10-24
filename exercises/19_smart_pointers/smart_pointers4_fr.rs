// Cet exercice explore le pointeur intelligent `Cow` (Clone-On-Write, « clone à l'écriture »). Il peut
// encapsuler des données empruntées et en fournir un accès immuable, et ne cloner les données
// que paresseusement, quand une mutation ou la propriété est nécessaire. Le type est conçu pour fonctionner
// avec des données empruntées générales via le trait `Borrow`.

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // Clone dans un vecteur s'il n'est pas déjà possédé.
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // Un clone a lieu car `input` doit être muté.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // Aucun clone n'a lieu car `input` n'a pas besoin d'être muté.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO : Remplacez `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // On peut aussi passer `vec` sans `&`, de sorte que `Cow` le possède directement. Dans ce
        // cas, aucune mutation n'a lieu (tous les nombres sont déjà positifs) et donc
        // aucun clone non plus. Mais le résultat est quand même possédé, car il n'a jamais été
        // emprunté ni muté.
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO : Remplacez `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // Bien sûr, c'est aussi le cas si une mutation a lieu (tous les
        // nombres ne sont pas positifs). Dans ce cas, l'appel à `to_mut()` dans la
        // fonction `abs_all` renvoie une référence vers les mêmes données qu'avant.
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO : Remplacez `todo!()` par `Cow::Owned(_)` ou `Cow::Borrowed(_)`.
        assert!(matches!(input, todo!()));
    }
}
