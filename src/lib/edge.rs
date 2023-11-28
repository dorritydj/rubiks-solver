use crate::Colors;

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    color: Colors,
    edge: Colors,
    // corners: Vec<Corner>,
}

impl Edge {
    pub fn new(color: Colors, edge: Colors) -> Self {
        return Self { color, edge };
    }
}
