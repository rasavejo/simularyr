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

    pub ops_count: u64,
    pub mem_percent: f32,
    pub alu_percent: f32,
    pub flu_percent: f32,

    pub next: Option<Box<Task>>,
}
