mod task;
mod sched;
mod machine;

use crate::task::*;
use crate::sched::schedule;


fn main() {

    let task2 = Task{
        id : "t2".to_string(),
        ops_count: 1000,
        mem_percent: 0.,
        alu_percent: 1.,
        flu_percent: 0.,
        next: None
    };

    let task1 = Task{
        id : "t1".to_string(),
        ops_count: 1000,
        mem_percent: 0.,
        alu_percent: 1.,
        flu_percent: 0.,
        next: Some(Box::new(task2))
    };

}
