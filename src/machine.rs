use std::{cell::RefCell, cmp::max};

use crate::task::*;

// purely description
pub struct Cpu<'a> {
    pub id: &'a str,

    pub alu: Alu,
    pub fpu: Fpu,
    pub cache : Cache<'a>,
    pub ram : &'a RefCell<Ram>
}

impl Cpu<'_> {
    pub fn run_task(&mut self, task:Task,time:u32) -> u32 {
        let time_alu = self.alu.run_task(task);
        let time_mem = self.mem_access(task,time);
        let time_fpu = self.fpu.run_task(task);
        max(max(time_alu,time_mem),time_fpu)
    }


    fn mem_access(&mut self, task:Task,time: u32) -> u32 {
        if task.mem_count == 0 {return 0}
        let mut total_time:u32 = 0;
        let nb_miss = (task.mem_count as f32 * task.cache_miss) as u32;
        total_time += self.cache.access_cache(task.mem_count-nb_miss,task.l1_cache_miss,task.l2_cache_miss,time);
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

pub struct Fpu {
    pub op_duration : u32,
    pub nb_of_fpu : u32,
}

impl Fpu {
    fn run_task(&self, task:Task) -> u32 {
        if task.fpu_count == 0 {return 0}
        let nb_op = task.fpu_count;
        let time_until_end = (nb_op as u32 / self.nb_of_fpu) * self.op_duration;
        time_until_end
    }
}


pub struct L3 {
    l3_cache_size : u32,
    l3_block_size : u32,
    l3_cache_access_duration : u32,
    cache_free:u32
}

impl L3 {
    pub fn new(l3_cache_size:u32,l3_block_size:u32,l3_cache_access_duration:u32) -> L3 {
        L3 {l3_cache_size, l3_block_size, l3_cache_access_duration,cache_free:0}
    }

    pub fn access_l3(&mut self,nb_access:u32,time:u32) -> u32 {
        let mut total_time = 0;
        if self.cache_free > time {
            total_time += self.cache_free - time;
        } else {
            self.cache_free = time;
        }
        self.cache_free += self.l3_cache_access_duration * nb_access;
        total_time + (self.l3_cache_access_duration * nb_access)
    }
}


pub struct Cache<'a> {
    pub l1_cache_access_duration : u32,
    pub l2_cache_access_duration : u32,
    pub l1_cache_size : u32,
    pub l2_cache_size : u32,
    pub l1_block_size : u32,
    pub l2_block_size : u32,
    pub l3 : &'a RefCell<L3>
}

impl Cache<'_> {
    fn access_cache(&self,nb_access:u32,l1_miss:f32,l2_miss:f32,time:u32) -> u32 {
        let nb_l1_miss =  (l1_miss * nb_access as f32) as u32;
        let nb_l3 = (l2_miss * nb_l1_miss as f32) as u32;
        let nb_l2 = nb_l1_miss - nb_l3;
        let nb_l1 = nb_access - nb_l1_miss;
        self.l3.borrow_mut().access_l3(nb_l3,time) + (nb_l2 * self.l2_cache_access_duration) + (nb_l1 * self.l1_cache_access_duration) 
    }
}


pub struct Ram {
    ram_access_duration : u32,
    ram_free:u32,
}

impl Ram {
    pub fn new(ram_access_duration : u32) -> Ram {
        Ram { ram_access_duration, ram_free: 0 }
    }

    pub fn access_ram(&mut self,nb_access:u32,time:u32) -> u32 {
        let mut total_time = 0;
        if self.ram_free > time {
            total_time += self.ram_free - time;
        } else {
            self.ram_free = time;
        }
        self.ram_free += self.ram_access_duration * nb_access;
        total_time + (self.ram_access_duration * nb_access)
    }
}