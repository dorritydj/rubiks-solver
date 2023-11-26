#[derive(Debug)]
pub struct Face {
  pub color: char,
  // corners: [Corner; 4],
  // edges: [Edge; 4],
}

impl Face {
  pub fn new(color: char) -> Self {
    return Self {
      color
    }
  }
}