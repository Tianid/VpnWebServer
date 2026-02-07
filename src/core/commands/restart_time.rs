#[derive(Clone, Copy)]
pub enum RestartTime {
    Now,
    AfterSeconds(u32)
}
