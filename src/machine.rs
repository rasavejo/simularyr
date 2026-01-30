use std::cell::RefCell;

use crate::task::*;

// purely description
pub struct Cpu<'a> {
    pub id: &'a str,

    pub alu: Alu,
    // pub fpu: Fpu,
    pub cache : Cache,
    pub ram : &'a RefCell<Ram>
}

impl Cpu<'_> {
    pub fn run_task(&mut self, task:Task,time:u32) -> u32 {
        let time_alu = self.alu.run_task(task);
        let time_mem = self.mem_access(task,time);

        println!("TEMPS MEM {time_mem}");

        if time_mem > time_alu {
            time_mem
        } else {
            time_alu
        }
    }


    fn mem_access(&mut self, task:Task,time: u32) -> u32 {
        if task.mem_count == 0 {return 0}
        let mut total_time:u32 = 0;
        let nb_miss = (task.mem_count as f32 * task.cache_miss) as u32;
        total_time += self.cache.access_cache(task.mem_count-nb_miss);
        total_time += self.ram.borrow_mut().access_ram(nb_miss,time);
        total_time
    }


}

pub struct Alu {
    pub ops_per_cycle : u32,
    pub nb_of_alu : u32,
}

impl Alu {
    fn run_task(&self, task:Task) -> u32 {
        if task.alu_count == 0 {return 0}
        let nb_op = task.alu_count;
        let time_until_end = (nb_op as u32 / self.nb_of_alu) / self.ops_per_cycle;
        time_until_end
    }
}

struct Fpu {

}

pub struct Cache {
    pub cache_access_duration : u32,
}

impl Cache {
    fn access_cache(&self,nb_access:u32) -> u32 {
        self.cache_access_duration * nb_access
    }
}


pub struct Ram {
    pub ram_access_duration : u32,
    pub ram_free:u32,
}

impl Ram {
    pub fn new(ram_access_duration : u32) -> Ram {
        Ram { ram_access_duration, ram_free: 0 }
    }

    pub fn access_ram(&mut self,nb_access:u32,time:u32) -> u32 {
        let mut total_time = 0;
        println!("RAM FREE AT {:?}",self.ram_free);
        if self.ram_free > time {
            total_time += self.ram_free - time;
        } else {
            self.ram_free = time;
        }
        self.ram_free += self.ram_access_duration * nb_access;
        println!("TIME ACCES MEM {total_time}");
        total_time + (self.ram_access_duration * nb_access)
    }
}