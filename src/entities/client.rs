use std::hash::{Hash, Hasher};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    pub(crate) id: String,
    pub(crate) stream: TcpStream,
}

impl Client {
    pub fn new(id: String, stream: TcpStream) -> Self { Self { id, stream } }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Client {}

impl Hash for Client {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
