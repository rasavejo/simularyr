use crate::task::*;

mod cpu;
mod mem;

use crate::machine::cpu::cpu_update;
use crate::machine::cpu::add_cpu_tasks;
use crate::machine::mem::mem_update;
use crate::machine::mem::add_mem_tasks;

let queue : Vec<Task> = Vec::new();
//TODO : Faire une globale idiomatique au rust
//Globale obligatoire quoiqu'il arrive parce qu'il faut qu'à chaque appel de la fonction la fonction se rappelle où elle en est de l'execution

fn add_tasks(new_tasks:Vec<Task>) {
    queue.append(new_tasks);
}

fn update(mut finished_tasks:Vec<Task>,time:u64) {
    let mut updated_tasks: Vec<Task> = Vec::new();
    cpu_update(updated_tasks,time);
    mem_update(updated_tasks, time);
    updated_tasks.append(queue);
    queue.clear();
    for t in updated_tasks {
        if t.instructions.is_empty() {
            finished_tasks.push(t);

        }
            // TODO : Switch qui teste la prochaine instruction et appelle cpu ou mem selon
    }

}