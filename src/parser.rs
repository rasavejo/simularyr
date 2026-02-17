use serde_json::*;
use crate::{machine::*, task::Task};
use std::{cell::RefCell};

pub fn parse_ram(v:&Value) -> RefCell<Ram> {
    let err = "Error while parsing the Ram, see example-machine.json";
    let ram : Ram = Ram::new(v["ram"]["access_duration"]
        .as_u64().expect(&format!("access_duration missing : {err}")));
    RefCell::new(ram)
}

pub fn parse_l3(v:&Value) -> RefCell<L3> {
    let err = "Error while parsing the L3 cache, see example-machine.json";
    let l3 = L3::new(
        v["L3"]["l3_cache_size"].as_u64().expect(&format!("l3_cache_size missing : {err}")),
        v["L3"]["l3_block_size"].as_u64().expect(&format!("l3_block_size missing : {err}")),
        v["L3"]["l3_cache_access_duration"].as_u64().expect(&format!("l3_cache_access_duration missing : {err}")));
    RefCell::new(l3)
}

pub fn parse<'a>(v:&'a Value,ram:&'a RefCell<Ram>,l3:&'a RefCell<L3>) -> Machine<'a> {
    let err = "Error while parsing the cpu, see example-machine.json";
    let err_cache = "Error while parsing the cache, see example-machine.json";

    let mut cpus: Vec<Cpu> = Vec::new();

    for cpu in v["cpu"].as_array().expect(err) {
        cpus.push(Cpu { id: cpu["id"].as_str().expect(&format!("id missing : {err}")),
            alu: Alu { ops_per_cycle: {if let Some(ops) = cpu["alu_ops_per_cycle"].as_u64() {ops} else {1}},
                nb_of_alu: cpu["alu_count"].as_u64().expect(&format!("alu_count missing : {err}"))},
            fpu: Fpu { op_duration: cpu["fpu_op_duration"].as_u64().expect(&format!("fpu_op_duration missing : {err}")), 
                nb_of_fpu: cpu["fpu_count"].as_u64().expect(&format!("fpu_count missing : {err}"))},
            cache: Cache { 
                l1_cache_access_duration: cpu["l1_cache_access_duration"].as_u64().expect(&format!("l1_cache_access_duration missing : {err_cache}")),
                l2_cache_access_duration: cpu["l2_cache_access_duration"].as_u64().expect(&format!("l2_cache_access_duration missing : {err_cache}")),
                l1_cache_size: cpu["l1_cache_size"].as_u64().expect(&format!("l1_cache_size missing : {err_cache}")),
                l2_cache_size: cpu["l2_cache_size"].as_u64().expect(&format!("l2_cache_size missing : {err_cache}")),
                l1_block_size: cpu["l1_block_size"].as_u64().expect(&format!("l1_block_size missing : {err_cache}")),
                l2_block_size: cpu["l2_block_size"].as_u64().expect(&format!("l2_block_size missing : {err_cache}")),
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
            id: task["id"].as_str().expect(&format!("id missing : {err}")),
            dep: {if let Some(dep) = task["dep"].as_array() {    
                    let mut r = Vec::new();
                    for value in dep {
                        r.push(value.as_str().unwrap());
                    }
                    r
                } else {Vec::new()}},
            start_time: {if let Some(time) = task["start_time"].as_u64() {time} else {0}},            
            mem_op_count: {if let Some(mem) = task["mem_op_count"].as_u64() {mem} else {0}},
            alu_op_count: {if let Some(alu) = task["alu_op_count"].as_u64() {alu} else {0}},
            fpu_op_count: {if let Some(fpu) = task["fpu_op_count"].as_u64() {fpu} else {0}},
            cache_miss: task["cache_miss"].as_f64().expect(&format!("cache_miss (between 0.0 and 1.0) missing : {err}")),
            l1_cache_miss: task["l1_cache_miss"].as_f64().expect(&format!("l1_cache_miss (between 0.0 and 1.0) missing : {err}")),
            l2_cache_miss: task["l2_cache_miss"].as_f64().expect(&format!("l2_cache_miss (between 0.0 and 1.0) missing : {err}")),
        });
    }
    tasks
}