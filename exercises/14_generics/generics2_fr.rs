// Ce puissant wrapper permet de stocker une valeur entière positive.
// TODO : Réécrivez-le avec un générique pour qu'il puisse encapsuler N'IMPORTE QUEL type.
struct Wrapper {
    value: u32,
}

// TODO : Adaptez l'implémentation de la struct pour qu'elle soit générique sur la valeur encapsulée.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
