// AsRef et AsMut permettent des conversions de référence à référence peu coûteuses. Pour en
// savoir plus, voir https://doc.rust-lang.org/std/convert/trait.AsRef.html et
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectivement.

// Obtenez le nombre d'octets (et non de caractères) de l'argument donné
// (`.len()` renvoie le nombre d'octets d'une chaîne).
// TODO : Ajoutez le trait `AsRef` de façon appropriée comme borne de trait.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtenez le nombre de caractères (et non d'octets) de l'argument donné.
// TODO : Ajoutez le trait `AsRef` de façon appropriée comme borne de trait.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Élève un nombre au carré avec `as_mut()`.
// TODO : Ajoutez la borne de trait appropriée.
fn num_sq<T>(arg: &mut T) {
    // TODO : Implémentez le corps de la fonction.
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
