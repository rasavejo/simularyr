mod task;
mod sched;
mod machine;
mod parser;
mod test;

use parser::*;
use crate::task::*;
// use crate::dag::*;


pub fn simulate(machine:&str,task:&str) -> u64 {
    let contents = std::fs::read_to_string(machine)
    .expect("Unable to read the machine desc file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let ram = parse_ram(&v);
    let l3 = parse_l3(&v);
    let mut machine = parse(&v,&ram,&l3);


    let contents = std::fs::read_to_string(task)
    .expect("Unable to read the task desc file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let tasks : Vec<Task> = parse_tasks(&v);

    machine.schedule(&tasks)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        panic!("USAGE : cargo run <machine desc json> <task desc json>");
    }

    println!("{:?}",simulate(&args[1], &args[2]));
}
