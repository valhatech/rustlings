// Dans cet exercice, nous allons implémenter `FromStr` pour convertir des données stockées sous
// forme de chaîne en un type structuré. Contrairement au trait `From`, la conversion
// exprimée par `FromStr` peut échouer, elle renvoie donc un `Result`. De plus,
// une fois `FromStr` implémenté, vous pouvez utiliser la méthode `parse` sur les chaînes pour
// générer un objet du type implémenteur. Vous pouvez en lire plus dans
// la documentation :
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// Nous utiliserons ce type d'erreur pour l'implémentation de `FromStr`.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Nombre de champs incorrect
    BadLen,
    // Champ du nom vide
    NoName,
    // Erreur encapsulée issue de parse::<u8>()
    ParseInt(ParseIntError),
}

// TODO : Complétez cette implémentation de `FromStr` pour pouvoir analyser une `Person`
// à partir d'une chaîne de la forme "Mark,20".
// Notez que vous devrez convertir la partie âge en `u8` avec quelque chose
// comme `"4".parse::<u8>()`.
//
// Étapes :
// 1. Découpez la chaîne donnée selon les virgules qu'elle contient.
// 2. Si l'opération de découpage renvoie moins ou plus de 2 éléments, renvoyez
//    l'erreur `ParsePersonError::BadLen`.
// 3. Utilisez le premier élément du découpage comme nom.
// 4. Si le nom est vide, renvoyez l'erreur `ParsePersonError::NoName`.
// 5. Convertissez le deuxième élément du découpage en `u8` pour obtenir l'âge.
// 6. Si la conversion de l'âge échoue, renvoyez l'erreur `ParsePersonError::ParseInt`.
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
