use std::collections::HashMap;
use std::u32::{MAX, MIN};

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
    pub fn next_free_cpu(vec:&Vec<u32>) -> (usize,u32){
        let mut mini: u32 = MAX;
        let mut index: usize = 0;
        for i in 0..vec.len() {
            if vec[i] < mini {
                index = i;
                mini = vec[i];
            }
        }
        (index,mini)
    }

    pub fn schedule(&self,tasks:Vec<Task>) -> u32 {
        let mut time = 0;

        let mut time_until_next: Vec<u32> = Vec::new();
        for _i in 0..self.cpus.len() {
            time_until_next.push(0);
        }


        for task in tasks {
            let (next_cpu,next_date) = Self::next_free_cpu(&time_until_next);
            println!("Next CPU : {next_cpu} and next_date : {next_date}");

            if next_date != time  {
                time = next_date;
            }
            time_until_next[next_cpu] += self.cpus[next_cpu].run_task(task);
        }
        *time_until_next.iter().max().unwrap()
    }
}
