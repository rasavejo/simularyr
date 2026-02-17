#[derive(Clone)]
pub struct Task<'a> {
    pub id : &'a str,

    pub dep: Vec<&'a str>,
    pub start_time: u64,

    pub mem_op_count: u64,
    pub alu_op_count: u64,
    pub fpu_op_count: u64,

    pub cache_miss: f64,
    pub l1_cache_miss: f64,
    pub l2_cache_miss: f64
}
