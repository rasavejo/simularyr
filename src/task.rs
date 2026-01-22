#[derive(Clone, Copy)]
pub struct Task<'a> {
    pub id : &'a str,

    pub ops_count: u64,
    pub mem_percent: f32,
    pub alu_percent: f32,
    pub flu_percent: f32,
}
