use std::collections::HashMap;

use crate::task::*;


let mut queue: Vec<(Task,u64)> = HashMap::new();
//TODO : Faire une globale idiomatique au rust


// TODO : Faire plusieurs coeurs (sinon c'est nul)

pub fn add_cpu_tasks(new_tasks:Vec<Task>,) {
    for t in new_tasks {
        queue.push(new_tasks,0);
    }
}

pub fn cpu_update(mut finished_tasks:Vec<Task>,time:u64) {
    if !queue.is_empty() {
        if queue.get(0).1 == 0 {
            queue.get(0).1 += 1;            //TODO : Ajouter vrai temps de calcul.
        } else if queue.get(0).1 == time {
            finished_tasks.push(t);
        }
    }
}
