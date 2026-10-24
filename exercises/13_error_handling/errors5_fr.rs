// Cet exercice est une version modifiée de l'exercice `errors4`. Il utilise quelques
// notions que nous n'aborderons que plus tard dans le cours, comme `Box` et le
// trait `From`. Il n'est pas important de les comprendre en détail pour l'instant, mais
// vous pouvez lire la suite si vous le souhaitez. Pour l'instant, considérez le type `Box<dyn ???>` comme
// un type « je veux n'importe quoi qui fait ??? ».
//
// En bref, cet usage particulier des box sert quand on veut posséder une
// valeur et que l'on se soucie seulement qu'elle soit d'un type qui implémente un
// trait donné. Pour cela, la `Box` est déclarée de type `Box<dyn Trait>` où
// `Trait` est le trait que le compilateur recherche sur toute valeur utilisée dans ce
// contexte. Pour cet exercice, ce contexte correspond aux erreurs potentielles
// qui peuvent être renvoyées dans un `Result`.

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// Ceci est nécessaire pour que `CreationError` puisse implémenter `Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO : Ajoutez le bon type de retour `Result<(), Box<dyn ???>>`. Que peut-on
// utiliser pour décrire les deux erreurs ? Existe-t-il un trait que les deux erreurs implémentent ?
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
