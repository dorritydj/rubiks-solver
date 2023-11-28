// k is the size of the combination
// should probably assume all combos should be unique
// maybe make it a hashmap???
// W: R,O,G,B
// R: W,Y,R,O
// then you can iterate over the keys, and make pairs from the values?
// still n^2 though
pub fn combinations(vec: &Vec<T>, k: usize) -> Vec<T> {
  let mut holder = Vec::new();
}