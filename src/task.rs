pub enum InstType {
    ADD,
    MUL,
    LDR,
    STR
}

pub struct Inst {
    pub instruction: InstType,
    pub address: u64   
}

pub struct Task {
    pub id: String,
    pub duration: u64,
    pub submit_time : u64,
    pub instructions : Vec<Inst>
}