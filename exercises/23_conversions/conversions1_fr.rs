// La conversion de types en Rust se fait avec l'opérateur `as`.
// Notez que l'opérateur `as` ne sert pas qu'à convertir des types. Il permet aussi
// de renommer des imports.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO : Faites une conversion avant de diviser.
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
