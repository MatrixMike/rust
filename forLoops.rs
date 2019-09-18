// nested for loops
fn main() {
    let mut arr = [0; 16];
    let slice: &mut [i32] = &mut arr;

    let slice_len = slice.len();
    for i in 2..slice_len {
        for j in (i * 2)..slice_len {
            println!("i: {:?}, j: {:?}", i, j);
            // <your mutation here>
        }
    }
}
