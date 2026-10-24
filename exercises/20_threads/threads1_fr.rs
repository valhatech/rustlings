// Ce programme lance plusieurs threads qui s'exécutent chacun pendant au moins 250 ms, et
// chaque thread renvoie le temps qu'il a mis à se terminer. Le programme doit
// attendre que tous les threads lancés soient terminés et doit collecter leurs
// valeurs de retour dans un vecteur.

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO : Collectez les résultats de tous les threads dans le vecteur `results`.
        // Utilisez la struct `JoinHandle` renvoyée par `thread::spawn`.
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
