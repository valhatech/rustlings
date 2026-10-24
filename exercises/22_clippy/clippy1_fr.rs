// L'outil Clippy est un ensemble de « lints » qui analysent votre code pour que vous puissiez
// repérer les erreurs courantes et améliorer votre code Rust.
//
// Pour ces exercices, le code ne compile pas tant qu'il y a des avertissements Clippy.
// Consultez les suggestions de Clippy dans la sortie pour résoudre l'exercice.

fn main() {
    // TODO : Corrigez le lint Clippy sur cette ligne.
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
