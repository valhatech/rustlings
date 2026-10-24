// On vous donne une liste de scores (un par ligne) de matchs de football. Chaque ligne a
// la forme "<nom_équipe_1>,<nom_équipe_2>,<buts_équipe_1>,<buts_équipe_2>"
// Exemple : "England,France,4,2" (l'Angleterre a marqué 4 buts, la France 2).
//
// Vous devez construire un tableau des scores contenant le nom de l'équipe, le
// nombre total de buts marqués par l'équipe et le nombre total de buts
// encaissés par l'équipe.

use std::collections::HashMap;

// Une structure pour stocker le détail des buts d'une équipe.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_score_table(results: &str) -> HashMap<&str, TeamScores> {
    // Le nom de l'équipe est la clé et la struct associée est la valeur.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // NOTE : On utilise `unwrap` parce qu'on n'a pas encore abordé la gestion des erreurs.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO : Remplissez le tableau des scores avec les informations extraites.
        // Gardez à l'esprit que les buts marqués par l'équipe 1 correspondent aux buts
        // encaissés par l'équipe 2. De même, les buts marqués par l'équipe 2 correspondent
        // aux buts encaissés par l'équipe 1.
    }

    scores
}

fn main() {
    // Vous pouvez expérimenter ici si vous le souhaitez.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_score_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
