#[derive(Clone, Copy)]
pub struct Task<'a> {
    pub id : &'a str,

    pub mem_count: u32,
    pub alu_count: u32,
    pub fpu_count: u32,

    pub cache_miss: f32,
}
