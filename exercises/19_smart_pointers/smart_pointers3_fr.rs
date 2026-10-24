// Dans cet exercice, on nous donne un `Vec` de `u32` appelé `numbers` dont les valeurs
// vont de 0 à 99. Nous voulons utiliser cet ensemble de nombres dans 8
// threads différents simultanément. Chaque thread va calculer la somme
// d'une valeur sur huit, avec un décalage.
//
// Le premier thread (décalage 0) fera la somme de 0, 8, 16, …
// Le deuxième thread (décalage 1) fera la somme de 1, 9, 17, …
// Le troisième thread (décalage 2) fera la somme de 2, 10, 18, …
// …
// Le huitième thread (décalage 7) fera la somme de 7, 15, 23, …
//
// Chaque thread doit posséder un pointeur à comptage de références vers le vecteur
// de nombres. Mais `Rc` n'est pas thread-safe. Nous devons donc utiliser `Arc`.
//
// Ne vous laissez pas distraire par la façon dont les threads sont lancés et joints. Nous nous y
// exercerons plus tard dans les exercices sur les threads.

// Ne modifiez pas les lignes ci-dessous.
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO : Définissez `shared_numbers` en utilisant `Arc`.
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO : Définissez `child_numbers` à partir de `shared_numbers`.
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
