use rubiks::*;

fn main() {
    let cube = Cube::new(3);

    println!("{:?}", cube.get_face('W'));
}
