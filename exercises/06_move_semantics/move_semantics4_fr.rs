fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    // TODO : Corrigez les erreurs de compilation uniquement en réordonnant les lignes du test.
    // N'ajoutez, ne modifiez et ne supprimez aucune ligne.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
