// Ceci est un quiz sur les sections suivantes :
// - Chaînes de caractères
// - Vecs
// - Sémantique de déplacement
// - Modules
// - Enums
//
// Construisons une petite machine sous la forme d'une fonction. En entrée, nous allons
// lui donner une liste de chaînes et de commandes. Ces commandes déterminent quelle action
// sera appliquée à la chaîne. Ce peut être :
// - Mettre la chaîne en majuscules
// - Supprimer les espaces en début et fin de chaîne (trim)
// - Ajouter "bar" à la chaîne un nombre de fois donné
//
// La forme exacte sera la suivante :
// - L'entrée sera un Vecteur de tuples de longueur 2,
//   le premier élément est la chaîne, le second la commande.
// - L'élément de sortie sera un vecteur de chaînes.

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO : Complétez la fonction comme décrit ci-dessus.
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    // TODO : Que faut-il importer pour avoir `transformer` dans la portée ?
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
