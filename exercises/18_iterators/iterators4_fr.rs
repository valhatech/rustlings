fn factorial(num: u64) -> u64 {
    // TODO : Complétez cette fonction pour qu'elle renvoie la factorielle de `num`, définie
    // par `1 * 2 * 3 * … * num`.
    // https://fr.wikipedia.org/wiki/Factorielle
    //
    // N'utilisez pas :
    // - de retours anticipés (utilisation explicite du mot-clé `return`)
    // Essayez de ne pas utiliser :
    // - de boucles de style impératif (for/while)
    // - de variables supplémentaires
    // Pour un défi supplémentaire, n'utilisez pas :
    // - la récursion
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
