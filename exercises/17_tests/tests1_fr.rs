// Les tests sont importants pour s'assurer que votre code fait ce que vous pensez qu'il
// doit faire.

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    // TODO : Importez `is_even`. Vous pouvez utiliser un joker (wildcard) pour importer tout ce qui est dans
    // le module parent.

    #[test]
    fn you_can_assert() {
        // TODO : Testez la fonction `is_even` avec quelques valeurs.
        assert!();
        assert!();
    }
}
