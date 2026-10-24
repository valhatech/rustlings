// Dans cet exercice, nous voulons exprimer le concept de propriétaires multiples via le
// type `Rc<T>`. C'est un modèle de notre système solaire — il y a un type `Sun` et
// plusieurs `Planet`. Les planètes prennent possession du soleil, ce qui indique
// qu'elles tournent autour de lui.

use std::rc::Rc;

#[derive(Debug)]
struct Sun;

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {self:?}!");
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rc1() {
        let sun = Rc::new(Sun);
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 référence

        let mercury = Planet::Mercury(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 références
        mercury.details();

        let venus = Planet::Venus(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 références
        venus.details();

        let earth = Planet::Earth(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 références
        earth.details();

        let mars = Planet::Mars(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 références
        mars.details();

        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 références
        jupiter.details();

        // TODO
        let saturn = Planet::Saturn(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 références
        saturn.details();

        // TODO
        let uranus = Planet::Uranus(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 références
        uranus.details();

        // TODO
        let neptune = Planet::Neptune(Rc::new(Sun));
        println!("reference count = {}", Rc::strong_count(&sun)); // 9 références
        neptune.details();

        assert_eq!(Rc::strong_count(&sun), 9);

        drop(neptune);
        println!("reference count = {}", Rc::strong_count(&sun)); // 8 références

        drop(uranus);
        println!("reference count = {}", Rc::strong_count(&sun)); // 7 références

        drop(saturn);
        println!("reference count = {}", Rc::strong_count(&sun)); // 6 références

        drop(jupiter);
        println!("reference count = {}", Rc::strong_count(&sun)); // 5 références

        drop(mars);
        println!("reference count = {}", Rc::strong_count(&sun)); // 4 références

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 3 références

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 2 références

        // TODO
        println!("reference count = {}", Rc::strong_count(&sun)); // 1 référence

        assert_eq!(Rc::strong_count(&sun), 1);
    }
}
