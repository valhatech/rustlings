fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO : Extrayez du tableau `a` une slice nommée `nice_slice` pour que le test passe.
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
