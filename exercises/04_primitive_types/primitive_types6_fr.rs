fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO : Utilisez un index de tuple pour accéder au deuxième élément de `numbers`
        // et l'affecter à une variable nommée `second`.
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
