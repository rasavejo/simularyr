mod task;
mod sched;
mod machine;

use crate::task::*;
use crate::sched::schedule;


fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    tasks.push(Task{instructions:vec![Inst{instruction:InstType::ADD,address:0}],id:"t1".to_string(),duration:1,submit_time:1});
    tasks.push(Task{instructions:vec![Inst{instruction:InstType::LDR,address:0x4000}],id:"t2".to_string(),duration:1,submit_time:1});

    schedule(tasks);
}