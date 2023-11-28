use rubiks::*;

fn main() {
    let cube = Cube::new(3);

    println!("\n{:?}", cube.get_face(Colors::Color('W')));
}
