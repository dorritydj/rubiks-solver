use crate::Colors;
use crate::Face;
use crate::COLORS;

// opposite faces have the same adjacencies

#[derive(Debug)]
pub struct Cube<'a> {
    pub length: u8,
    pub faces: Vec<Face<'a>>,
}

// TODO:
// - create all combinations of corners/edges here
// - pass the appropriate references into the face when it's created
//      - this might be unnecessary/wonky given ownership
//      - cube focusing on it may be better/more simple for rotations
impl<'a> Cube<'a> {
    pub fn new(length: u8) -> Self {
        let faces = Self::create_faces();
        let connected_faces = Self::connect_faces(faces);

        return Self {
            length,
            faces: connected_faces,
        };
    }

    // pub fn get_face(&self, color: Colors) -> Option<&Face> {
    //     let index = self
    //         .faces
    //         .iter()
    //         .position(|face| face.color == color)
    //         .unwrap();
    //     return self.faces.get(index);
    // }

    // fn create_faces() -> Vec<Face<'a>> {
    //     let mut faces: Vec<Face> = Vec::new();

    //     for color in COLORS {
    //         let face = Face::new(color);

    //         faces.push(face);
    //     }

    //     return faces;
    // }

    // fn connect_faces(faces: Vec<Face<'a>>) -> Vec<Face<'a>> {
    //     for face in faces.iter() {
    //         // println!("{:?}", face);
    //     }

    //     return faces;
    // }
}
