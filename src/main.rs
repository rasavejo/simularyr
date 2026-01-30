mod task;
mod sched;
mod machine;
mod dag;

use std::cell::RefCell;

use crate::task::*;
use crate::sched::Sched;
use crate::machine::*;
// use crate::dag::*;
fn main() {

    let mem= RefCell::new(Ram::new(5));

    let cpu = Cpu {
        id: "1",
        alu: Alu{ ops_per_cycle: 1, nb_of_alu : 1},
        cache: Cache { cache_access_duration: 1 },
        ram: &mem
    };

    let cpu2 = Cpu {
        id: "1",
        alu: Alu{ ops_per_cycle: 1, nb_of_alu : 1},
        cache: Cache { cache_access_duration: 1 },
        ram: &mem
    };

    let mut sched = Sched::new(vec![cpu,cpu2]);

    let task2 = Task{
        id : "t2",
        mem_count : 1000,
        alu_count : 1000,
        fpu_count : 0,
        cache_miss: 0.1

    };
    let task1 = Task{
        id : "t1",
        mem_count : 1000,
        alu_count : 1000,
        fpu_count : 0,
        cache_miss: 0.1
    };

    let tasks : Vec<Task> = vec![task1,task2];

    println!("{:?}",sched.schedule(tasks));

}
