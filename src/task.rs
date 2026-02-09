#[derive(Clone, Copy)]
pub struct Task<'a> {
    pub id : &'a str,

    pub mem_op_count: u64,
    pub alu_op_count: u64,
    pub fpu_op_count: u64,

    pub cache_miss: f64,
    pub l1_cache_miss: f64,
    pub l2_cache_miss: f64
}
