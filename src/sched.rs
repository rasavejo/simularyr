use crate::task::*;
use crate::machine::update;

pub fn schedule(mut tasks:Vec<Task>) {
    let mut time = 0;
    while !tasks.is_empty() {
        for t in tasks {
            // TODO : implémenter une méthode simple de scheduling
            // add_task(task choisis)
        }

        let mut finished_tasks: Vec<Task> = Vec::new();
        update(finished_tasks,time);

        for v in finished_tasks {
            tasks.retain(|&x| x.id != v.id);   //supprime les éléments en double
        }
        time = time+1;
    }
}