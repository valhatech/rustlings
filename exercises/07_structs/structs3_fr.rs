// Les structs contiennent des données, mais peuvent aussi avoir de la logique. Dans cet
// exercice, nous avons défini la struct `Fireworks` et quelques fonctions qui
// travaillent avec elle. Transformez ces fonctions libres en méthodes et en
// fonctions associées pour exprimer plus clairement cette relation dans le code.

#![deny(clippy::use_self)] // entraînez-vous à utiliser le type `Self`

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO : Transformez cette fonction en fonction associée de `Fireworks`.
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO : Transformez cette fonction en méthode de `Fireworks`.
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO : Transformez cette fonction en méthode de `Fireworks`.
fn start(fireworks: Fireworks) -> String {
    "🚀".repeat(fireworks.rockets)
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // On n'utilise pas la syntaxe de méthode dans le dernier test pour garantir que la
        // fonction `start` prend la propriété (ownership) du feu d'artifice.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}
