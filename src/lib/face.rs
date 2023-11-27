use crate::Colors;

#[derive(Debug)]
pub struct Face<'a> {
    pub color: Colors,
    pub adjacencies: Vec<&'a Colors>,
    // corners: [Corner; 4],
    // edges: [Edge; 4],
}

impl<'a> Face<'a> {
    pub fn new(color: Colors) -> Self {
        let (color, adjacencies) = Colors::connections(color);

        return Self {
            color,
            adjacencies,
        };
    }
}
