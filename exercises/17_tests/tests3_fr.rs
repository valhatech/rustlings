struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Ne modifiez pas cette fonction.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Renvoyer un `Result` serait préférable ici. Mais nous voulons apprendre
            // à tester des fonctions qui peuvent paniquer.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO : Ce test doit vérifier que le rectangle a la taille que nous
        // passons à son constructeur.
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // Vérifie la largeur
        assert_eq!(todo!(), 20); // Vérifie la hauteur
    }

    // TODO : Ce test doit vérifier que le programme panique quand on essaie de créer
    // un rectangle avec une largeur négative.
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO : Ce test doit vérifier que le programme panique quand on essaie de créer
    // un rectangle avec une hauteur négative.
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
