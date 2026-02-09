use std::u64::MAX;
use crate::task::*;
use crate::machine::*;


impl Machine<'_> {

    // Return the closest free cpu, like (cpu index in the cpus array,date when free) 
    pub fn next_free_cpu(vec:&Vec<u64>) -> (usize,u64){
        let mut mini: u64 = MAX;
        let mut index: usize = 0;
        for i in 0..vec.len() {
            if vec[i] < mini {
                index = i;
                mini = vec[i];
            }
        }
        (index,mini)
    }

    pub fn schedule(&mut self,tasks:Vec<Task>) -> u64 {
        let mut time = 0;

        let mut time_until_next: Vec<u64> = Vec::new();
        for _i in 0..self.cpus.len() {
            time_until_next.push(0);
        }


        for task in tasks {
            let (next_cpu,next_date) = Self::next_free_cpu(&time_until_next);

            if next_date != time  {
                time = next_date;
            }
            time_until_next[next_cpu] += self.cpus[next_cpu].run_task(task,time);
        }
        *time_until_next.iter().max().unwrap()
    }
}
