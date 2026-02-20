use std::cmp::max;
use std::collections::HashMap;
use std::u64::MAX;
use crate::task::*;
use crate::machine::*;
use std::collections::HashSet;


impl Machine<'_> {

    // Return the closest free cpu, like (cpu index in the cpus array,date when free) 
    fn next_free_cpu(vec:&Vec<u64>) -> (usize,u64){
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

    fn check_available_tasks<'a>(tasks:&'a Vec<Task<'a>>,available_tasks:&mut Vec<&'a Task<'a>>,finished_tasks:&HashSet<&str>) {
        available_tasks.clear();
        for t in tasks {
            if !finished_tasks.contains(t.id) {
                let add = t.dep.iter().all(|x| finished_tasks.contains(x));
                if add {
                    available_tasks.push(t);
                }
            }
        }
    }

    pub fn schedule(&mut self,tasks:&Vec<Task>) -> u64 {
        let mut time = 0;



        //A map linking the id of a task to the time it can be started.
        //Is used in particular for dependancies
        //We do not use the starting_time field in task to avoid a mutable reference
        //and to keep it as it can be a useful information.
        let mut time_until_next_task: HashMap<&str,u64> = HashMap::new();
        for task in tasks {
            time_until_next_task.insert(task.id, task.start_time);
        }

        //A vec containing ints, 
        //if vec[i] is 3 it means that the cpu i is available at time 3
        let mut time_until_next_cpu: Vec<u64> = Vec::new();
        for _i in 0..self.cpus.len() {
            time_until_next_cpu.push(0);
        }

        //A simple vec containing task available to run (no dependancies)
        //(is a vec cause we will need to sort it)
        let mut available_tasks = Vec::new();

        //A set containing all finished tasks
        let mut finished_tasks: HashSet<&str> = HashSet::new();


        Self::check_available_tasks(&tasks,&mut available_tasks,&finished_tasks);
        while !available_tasks.is_empty() {

            available_tasks.sort_by_key(|x| x.start_time);
            available_tasks.reverse();

            let task = available_tasks.pop().unwrap();
            let (next_cpu,next_date) = Self::next_free_cpu(&time_until_next_cpu);


            // We took the available task with the lowest starting time
            // and the next available cpu


            if time < next_date  {
                time = next_date;
            }

            if time < time_until_next_task[task.id] {
                time = time_until_next_task[task.id];
            }

            // We jumped in time, first at a time when the cpu is free,
            // then if needed we wait for the task to be ready

            println!("time is now : {:?}",time);

            for i in 0..time_until_next_cpu.len() {
                time_until_next_cpu[i] = max(time_until_next_cpu[i],time);
            }

            // We adjusted the cpu times to current date if not in use
            // (will be useful to simplify computing time later)

            let end_time = self.cpus[next_cpu].run_task(&task,time);
            time_until_next_cpu[next_cpu] += end_time;
            println!("Finished task : {:?}, started at {:?} and ended at {:?}",task.id,time,time+end_time);
            println!("Made {:?} alu op, {:?} fpu op and {:?} mem access",task.alu_op_count,task.fpu_op_count,task.mem_op_count);

            // We ran a task

            finished_tasks.insert(task.id);
            for t in tasks {
                if t.dep.contains(&task.id) {
                    if time_until_next_task[t.id] < end_time{
                        time_until_next_task.insert(t.id, end_time);
                    }
                }
            }

            // We adjusted the starting times following dependancies



            Self::check_available_tasks(&tasks,&mut available_tasks,&finished_tasks);

            // we check the dependancies to add new tasks

        }
        *time_until_next_cpu.iter().max().unwrap()
    }
}
