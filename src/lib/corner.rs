use crate::Colors;

#[derive(Debug)]
pub struct Corner {
    color: Colors,
    corners: Vec<Colors>,
    // edges: [Edge; 2],
}

impl Corner {
    pub fn new(color: Colors, corners: Vec<Colors>) -> Self {
        return Self { color, corners };
    }
}
