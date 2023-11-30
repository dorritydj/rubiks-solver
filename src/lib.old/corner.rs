use crate::Colors;

#[derive(Debug)]
pub struct Corner {
    color: Colors,
    // corners: [Corner; 2],
    // edges: [Edge; 2],
}

impl Corner {
    pub fn new(color: Colors, corners: Vec<Colors>) -> Self {
        return Self {
            color,
            // corners
        };
    }
}
