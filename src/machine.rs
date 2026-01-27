use crate::task::*;



// purely description
pub struct Cpu<'a> {
    pub id: &'a str,

    pub alu: Alu,
    // pub fpu: Fpu,
    pub mem: Mem,
}

impl Cpu<'_> {
    pub fn run_task(&self, task:Task) -> u32 {
        let time_alu = self.alu.run_task(task);
        let time_mem = self.mem.run_task(task);

        if time_mem > time_alu {
            time_mem
        } else {
            time_alu
        }
    }
}

pub struct Alu {
    pub ops_per_cycle : u32,
    pub concurrent_ops : u32,
}

impl Alu {
    fn run_task(&self, task:Task) -> u32 {
        if task.alu_count == 0 {return 0}
        let nb_op = task.alu_count;
        let time_until_end = nb_op as u32 / self.ops_per_cycle;
        time_until_end
    }
}

struct Fpu {

}

pub struct Mem {
    pub access_duration : u32,
}

impl Mem {
    fn run_task(&self, task:Task) -> u32 {
        if task.mem_count == 0 {return 0}
        let nb_op = task.mem_count;
        let time_until_end = nb_op as u32 / self.access_duration;
        time_until_end + 1    // Durée dans le bus, à préciser
    }
}
