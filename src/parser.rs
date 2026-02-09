use serde_json::*;
use crate::{machine::*, task::Task};
use std::{cell::RefCell};

pub fn parse_ram(v:&Value) -> RefCell<Ram> {
    let err = "Error while parsing the Ram, see example-machine.json";
    let ram : Ram = Ram::new(v["ram"]["access_duration"]
        .as_u64().expect(err));
    RefCell::new(ram)
}

pub fn parse_l3(v:&Value) -> RefCell<L3> {
    let err = "Error while parsing the L3 cache, see example-machine.json";
    let l3 = L3::new(
        v["L3"]["l3_cache_size"].as_u64().expect(err),
        v["L3"]["l3_block_size"].as_u64().expect(err),
        v["L3"]["l3_cache_access_duration"].as_u64().expect(err));
    RefCell::new(l3)
}

pub fn parse<'a>(v:&'a Value,ram:&'a RefCell<Ram>,l3:&'a RefCell<L3>) -> Machine<'a> {
    let err = "Error while parsing the cpu, see example-machine.json";
    let err_cache = "Error while parsing the cache, see example-machine.json";

    let mut cpus: Vec<Cpu> = Vec::new();

    for cpu in v["cpu"].as_array().expect(err) {
        cpus.push(Cpu { id: cpu["id"].as_str().expect(err),
            alu: Alu { ops_per_cycle: cpu["alu_ops_per_cycle"].as_u64().expect(err),
                nb_of_alu: cpu["alu_count"].as_u64().expect(err)},
            fpu: Fpu { op_duration: cpu["fpu_op_duration"].as_u64().expect(err), 
                nb_of_fpu: cpu["fpu_count"].as_u64().expect(err)},
            cache: Cache { 
                l1_cache_access_duration: cpu["l1_cache_access_duration"].as_u64().expect(err_cache),
                l2_cache_access_duration: cpu["l2_cache_access_duration"].as_u64().expect(err_cache),
                l1_cache_size: cpu["l1_cache_size"].as_u64().expect(err_cache),
                l2_cache_size: cpu["l2_cache_size"].as_u64().expect(err_cache),
                l1_block_size: cpu["l1_block_size"].as_u64().expect(err_cache),
                l2_block_size: cpu["l2_block_size"].as_u64().expect(err_cache),
                l3: l3 },
            ram: ram });
    }

    Machine::new(cpus)
}

pub fn parse_tasks<'a>(v:&'a Value) -> Vec<Task<'a>> {
    let err = "Error while parsing the tasks, see example-task.json";
    let mut tasks = Vec::new();
    for task in v["tasks"].as_array().expect(err) {
        tasks.push(Task { 
            id: task["id"].as_str().expect(err),
            mem_op_count: task["mem_op_count"].as_u64().expect(err),
            alu_op_count: task["alu_op_count"].as_u64().expect(err),
            fpu_op_count: task["fpu_op_count"].as_u64().expect(err),
            cache_miss: task["cache_miss"].as_f64().expect(err),
            l1_cache_miss: task["l1_cache_miss"].as_f64().expect(err),
            l2_cache_miss: task["l2_cache_miss"].as_f64().expect(err),
        });
    }
    tasks
}