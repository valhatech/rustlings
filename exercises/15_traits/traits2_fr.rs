trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO : Implémentez le trait `AppendBar` pour un vecteur de chaînes.
// `append_bar` doit ajouter la chaîne "Bar" au vecteur.

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
