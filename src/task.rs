#[derive(Clone, Copy)]
pub struct Task<'a> {
    pub id : &'a str,

    pub mem_count: u64,
    pub alu_count: u64,
    pub fpu_count: u64,
}
