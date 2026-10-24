// TODO : Cette fonction refuse de générer le texte d'un badge nominatif si
// on lui passe une chaîne vide. Il serait plus agréable qu'elle explique quel
// est le problème au lieu de simplement renvoyer `None`. Heureusement, Rust possède une
// construction similaire à `Option` qui permet d'exprimer des conditions d'erreur. Modifiez
// la signature et le corps de la fonction pour qu'elle renvoie `Result<String, String>`
// au lieu de `Option<String>`.
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // Les noms vides ne sont pas autorisés
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
