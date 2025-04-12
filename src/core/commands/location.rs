#[derive(Clone, Copy)]
pub enum Location {
    DE,
    FI,
}

impl Location {
    pub fn get_name(&self) -> String {
        match self {
            Location::DE => String::from("DE"),
            Location::FI => String::from("FI"),
        }
    }
}


