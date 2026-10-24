// Alice est institutrice et doit calculer la note moyenne aux tests de
// trois classes dont elle s'occupe. Au lieu de les calculer l'une après
// l'autre, elle décide de demander de l'aide à ses amis Bob et Catherine. En travaillant
// ensemble, ils finissent le travail bien plus vite.
//
// Simulons cela avec la programmation asynchrone. Chaque personne est
// représentée par une tâche asynchrone, qui peut être exécutée de façon concurrente.

// Les tâches asynchrones doivent être exécutées par un « runtime », que la bibliothèque
// standard de Rust ne fournit pas. Ici, nous utilisons le runtime courant `tokio`.
// La macro `tokio::main` enveloppe toute la fonction main dans un runtime.
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    // TODO : Attendez (await) les tâches lancées pour vérifier leurs résultats.
    assert_eq!(mean_score_a, 84); // alice
    assert_eq!(mean_score_b, 89); // bob
    assert_eq!(mean_score_c, 76); // catherine
}

// TODO : Corrigez les erreurs de compilation en rendant la fonction lancée asynchrone.
fn calculate_mean_score(scores_file: &str) -> usize {
    // Lire le fichier de façon asynchrone
    let file = tokio::fs::read_to_string(scores_file).await.unwrap();

    // Initialiser la somme et le nombre de notes
    let mut sum = 0;
    let mut n = 0;
    for line in file.lines() {
        // Analyser chaque ligne comme une note
        let score = line.parse::<usize>().unwrap();
        sum += score;
        n += 1;
    }

    sum / n
}
