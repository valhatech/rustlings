struct ColorRegularStruct {
    // TODO : Ajoutez les champs attendus par le test `regular_structs`.
    // Quels types doivent avoir les champs ? Quelles sont les valeurs minimale et maximale des couleurs RGB ?
}

struct ColorTupleStruct(/* TODO : Ajoutez les champs attendus par le test `tuple_structs` */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO : Instanciez une struct classique.
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO : Instanciez une struct tuple.
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO : Instanciez une struct unité.
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
