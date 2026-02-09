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
    .expect("Should have been able to read the machine desc file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let ram = parse_ram(&v);
    let l3 = parse_l3(&v);
    let mut machine = parse(&v,&ram,&l3);


    let file_path = &args[2];
    let contents = std::fs::read_to_string(file_path)
    .expect("Should have been able to read the task desc file");
    let v: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let tasks : Vec<Task> = parse_tasks(&v);

    println!("{:?}",machine.schedule(tasks));
}
