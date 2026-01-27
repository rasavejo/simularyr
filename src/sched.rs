use std::collections::HashMap;

use crate::task::*;
use crate::machine::Cpu;


pub struct Sched<'a> {
    cpus :Vec<Cpu<'a>>,
}


impl Sched<'_> {

    pub fn new(cpus: Vec<Cpu>) -> Sched {
        Sched { cpus }
    }


    // Return the closest free cpu, like (cpu index in the cpus array,date when free) 
    pub fn next_free_cpu(map:&HashMap<usize,u32>) -> (usize,u32){
        let mut mini: u32 = 0;
        let mut index: usize = 0;
        for (k,v) in map {
            if *v < mini {
                index = *k;
                mini = *v;
            }
        }
        (index,mini)
    }

    pub fn schedule(&self,tasks:Vec<Task>) -> u32 {
        let mut time = 0;

        let mut time_until_next: HashMap<usize, u32> = HashMap::new();
        for i in 0..self.cpus.len() {
            time_until_next.insert(i,0);
        }


        for task in tasks {
            let (next_cpu,next_date) = Self::next_free_cpu(&time_until_next);
            println!("Next CPU : {next_cpu} and next_date : {next_date}");

            if next_date != time  {
                time = next_date;
            } else {
                let cpu = time_until_next.entry(next_cpu).or_default();
                *cpu += self.cpus[next_cpu].run_task(task);
            }
        }
        *time_until_next.values().max().unwrap()
    }
}
