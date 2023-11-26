// struct Corner {
//   corners: [Corner; 2],
//   edges: [Edge; 2],
// }

// struct Edge {
//   edge: Edge,
//   corners: [Corner; 2],
// }

use crate::Face;

#[derive(Debug)]
pub struct Cube {
  pub length: u8,
  pub faces: Vec<Face>,
}

impl Cube {
  pub fn new(length: u8) -> Self {
    let faces = Self::create_faces();

    return Self {
      length,
      faces
    }
  }

  fn create_faces() -> Vec<Face> {
    let mut faces: Vec<Face> = Vec::new();
    let colors: [char; 6] = ['W', 'G', 'Y', 'B', 'R', 'O'];

    for color in colors {
      let face = Face::new(color);

      faces.push(face);
    }

    return faces;
  }

  pub fn get_face(&self, color: char) -> Option<&Face> {
    let index = self.faces.iter().position(|face| face.color == color).unwrap();
    return self.faces.get(index);
  }
}