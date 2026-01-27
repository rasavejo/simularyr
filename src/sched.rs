use crate::task::*;
use crate::machine::Cpu;


pub struct Sched<'a> {
    cpu :Cpu<'a>,
}


impl Sched<'_> {

    pub fn new(cpu: Cpu) -> Sched {
        Sched { cpu }
    }

    pub fn schedule(&self,tasks:Vec<Task>) -> u32 {
        let mut time = 0;
        for task in tasks {
            time += self.cpu.run_task(task);
        }
        time
    }

}
