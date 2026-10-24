// Cette fonction renvoie la quantité de glace restant dans le frigo.
// S'il est avant 22h00 (système 24 heures), il reste 5 boules. À 22h00,
// quelqu'un mange tout, il ne reste donc plus de glace (valeur 0). Renvoyez `None` si
// `hour_of_day` est supérieur à 23.
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO : Complétez le corps de la fonction.
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO : Corrigez ce test. Comment obtenir la valeur contenue dans
        // l'Option ?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Ne modifiez pas cette ligne.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
