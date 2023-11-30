type Side = [[u8; 3]; 3];
type Cube = (Side, Side, Side, Side, Side, Side);

fn main() {
    // w
    let front: Side = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    // g
    let top: Side = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    // o
    let right: Side = [[2, 2, 2], [2, 2, 2], [2, 2, 2]];
    // b
    let bottom: Side = [[3, 3, 3], [3, 3, 3], [3, 3, 3]];
    // r
    let left: Side = [[4, 4, 4], [4, 4, 4], [4, 4, 4]];
    // y
    let back: Side = [[5, 5, 5], [5, 5, 5], [5, 5, 5]];

    let mut cube: Cube = (front, top, right, bottom, left, back);
    cube = rotate_clockwise(cube);

    println!("{:?}", cube);
}
// Notes:
// - for any given Side, side[1][1] should never move (center square)

// TODO: make generic for any side
fn rotate_clockwise(mut cube: Cube) -> Cube {
    let (mut front, mut top, mut right, mut bottom, mut left, _) = cube;

    // shift the front face
    let [mut top_f, mut mid_f, mut bot_f] = front;
    let (i_top_f, i_bot_f) = (top_f[2], bot_f[0]);

    top_f.rotate_right(1);
    top_f[0] = mid_f[0];
    mid_f[0] = i_bot_f;

    bot_f.rotate_left(1);
    bot_f[2] = mid_f[2];
    mid_f[2] = i_top_f;

    front = [top_f, mid_f, bot_f];
    // end shifting front face

    // adjust sides
    let top_r = top[2];
    let right_r = right.map(|row| row[0]);
    let bot_r = bottom[0];
    let left_r = left.map(|row| row[2]);

    top[2] = left_r;
    bottom[0] = right_r;

    right
        .iter_mut()
        .enumerate()
        .for_each(|(i, row)| row[0] = top_r[i]);

    left.iter_mut()
        .enumerate()
        .for_each(|(i, row)| row[2] = bot_r[i]);
    // done adjusting sides

    cube.0 = front;
    cube.1 = top;
    cube.2 = right;
    cube.3 = bottom;
    cube.4 = left;

    return cube;
}
