// En prolongeant l'exercice précédent, nous voulons que tous les threads terminent leur
// travail. Mais cette fois, les threads lancés doivent se charger de mettre à jour une
// valeur partagée : `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO : `Arc` ne suffit pas si l'on veut un état partagé **mutable**.
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO : Vous devez effectuer une action avant de mettre à jour une valeur partagée.
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // On attend que tous les jobs soient terminés.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO : Affichez la valeur de `JobStatus.jobs_done`.
    println!("Jobs done: {}", todo!());
}
