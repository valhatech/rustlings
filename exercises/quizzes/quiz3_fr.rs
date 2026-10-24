// Ce quiz teste :
// - Les génériques
// - Les traits
//
// Une école de magie imaginaire dispose d'un nouveau système de génération de bulletins
// écrit en Rust ! Actuellement, le système ne permet de créer que des bulletins où la
// note de l'élève est représentée numériquement (p. ex. 1.0 -> 5.5). Cependant, l'
// école délivre aussi des notes alphabétiques (A+ -> F-) et doit pouvoir
// imprimer les deux types de bulletins !
//
// Apportez les modifications nécessaires à la struct `ReportCard` et au bloc impl
// pour prendre en charge les bulletins alphabétiques en plus des bulletins numériques.

// TODO : Ajustez la struct comme décrit ci-dessus.
struct ReportCard {
    grade: f32,
    student_name: String,
    student_age: u8,
}

// TODO : Ajustez le bloc impl comme décrit ci-dessus.
impl ReportCard {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            self.student_name, self.student_age, self.grade,
        )
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
