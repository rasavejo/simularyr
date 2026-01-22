mod task;
mod sched;
mod machine;

use crate::task::*;
use crate::sched::Sched;
fn main() {

    let cpu = Cpu::new(); // not done
    let sched = Sched::new(cpu);

    let task2 = Task{
        id : "t2",
        ops_count: 1000,
        mem_percent: 0.,
        alu_percent: 1.,
        flu_percent: 0.,

    };

    let task1 = Task{
        id : "t1",
        ops_count: 1000,
        mem_percent: 0.,
        alu_percent: 1.,
        flu_percent: 0.,
    };
    let tasks : Vec<Task> = vec![task1,task2];
    
    sched.schedule(tasks);
    
}
