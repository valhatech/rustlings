// Le trait `From` sert aux conversions de valeur à valeur. Si `From` est
// implémenté, une implémentation de `Into` est fournie automatiquement.
// Vous pouvez en lire plus dans la documentation :
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// Représenter des unités de mesure par des types distincts est une pratique courante.
// Cela évite de mélanger accidentellement des valeurs d'unités de mesure différentes.

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO : Convertissez des Celsius en Fahrenheit. Ne vous souciez pas de la précision
    // des flottants. La formule est : F = C * 1.8 + 32
}

impl From<Fahrenheit> for Celsius {
    // TODO : Convertissez des Fahrenheit en Celsius.
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}
