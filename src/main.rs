mod task;
mod sched;
mod machine;
mod parser;

use parser::*;
use crate::task::*;
// use crate::dag::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1];
    let contents = std::fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
        let v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let ram = parse_ram(&v);
    let l3 = parse_l3(&v);
    let mut machine = parse(&v,&ram,&l3);

    println!("cpu 1 name : {}", v["cpu"][0]["id"]);

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

    println!("{:?}",machine.schedule(tasks));
}
