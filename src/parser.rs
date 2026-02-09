use serde_json::*;
use crate::machine::{Ram};
use std::fs;
use std::{cell::RefCell};

pub fn parse(file_path: &str) -> RefCell<Ram> {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
        let v: Value = serde_json::from_str(&contents).unwrap();
    println!("cpu 1 name : {}", v["cpu"][0]["id"]);

    let ram : Ram = Ram::new(1);

    RefCell::new(ram)
}
