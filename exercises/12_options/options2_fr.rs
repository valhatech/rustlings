fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO : Transformez ceci en une instruction if-let dont la valeur est `Some`.
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO : Transformez ceci en une instruction while-let. Rappelez-vous que `Vec::pop()`
        // ajoute une couche d'`Option` supplémentaire. Vous pouvez faire du filtrage par motifs
        // imbriqué dans les instructions if-let et while-let.
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
