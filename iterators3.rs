
#![allow(unused_variables)]
fn main() {
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    }
    println!(" result {} ", v1);
}
/* https://doc.rust-lang.org/book/ch13-02-iterators.html
 */
 

