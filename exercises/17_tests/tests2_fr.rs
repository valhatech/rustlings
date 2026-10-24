// Calcule la puissance de 2 à l'aide d'un décalage de bits.
// `1 << n` équivaut à « 2 puissance n ».
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO : Testez la fonction `power_of_2` avec quelques valeurs.
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
