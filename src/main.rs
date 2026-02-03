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
    let l3 = RefCell::new(L3::new(8000000, 64, 10));

    let cpu = Cpu {
        id: "1",
        alu: Alu{ ops_per_cycle: 1, nb_of_alu : 1},
        cache: Cache {l1_cache_access_duration: 1, l2_cache_access_duration: 3, l1_cache_size: 32000, l2_cache_size: 256000, l1_block_size:64,l2_block_size:64,l3:&l3},
        ram: &mem,
        fpu: Fpu{ op_duration : 3, nb_of_fpu : 1},
    };

    let cpu2 = Cpu {
        id: "1",
        alu: Alu{ ops_per_cycle: 1, nb_of_alu : 1},
        cache: Cache {l1_cache_access_duration: 1, l2_cache_access_duration: 3, l1_cache_size: 32000, l2_cache_size: 256000, l1_block_size:64,l2_block_size:64,l3:&l3},
        ram: &mem,
        fpu: Fpu{ op_duration : 3, nb_of_fpu : 1},
    };

    let mut sched = Sched::new(vec![cpu,cpu2]);

    let task2 = Task{
        id : "t2",
        mem_count : 1000,
        alu_count : 1000,
        fpu_count : 0,
        cache_miss: 0.1,
        l1_cache_miss: 0.5,
        l2_cache_miss : 0.1
    };
    let task1 = Task{
        id : "t1",
        mem_count : 1000,
        alu_count : 1000,
        fpu_count : 0,
        cache_miss: 0.1,
        l1_cache_miss: 0.5,
        l2_cache_miss: 0.1
    };

    let tasks : Vec<Task> = vec![task1,task2];

    println!("{:?}",sched.schedule(tasks));

}
