use crate::task::*;



// purely description
pub struct Cpu {
    id: String,

    alu: Alu,
    fpu: Fpu,
    mem: Mem,
}

impl Cpu {
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

struct Alu {
    ops_per_cycle : u32,
    concurrent_ops : u32,
}

impl Alu {
    fn run_task(&self, task:Task) -> u32 {
        let nb_op = (task.ops_count as f32) / task.alu_percent; 
        let time_until_end = nb_op as u32 / self.ops_per_cycle;
        time_until_end
    }
}

struct Fpu {

}

struct Mem {
    access_duration : u32,
}

impl Mem {
    fn run_task(&self, task:Task) -> u32 {
        let nb_op = (task.ops_count as f32) / task.mem_percent; 
        let time_until_end = nb_op as u32 / self.access_duration;
        time_until_end + 1    // Durée dans le bus, à préciser
    }
}
