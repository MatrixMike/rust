
#![allow(unused_variables)]
fn main() {
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

trait Foo {
	fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
}
fn some_func<T: Foo>(foo: T) {
	foo.iterator_sum();
}

//let it = iterator_sum();
//println!("{}",it);

for val in v1_iter {
    println!("Got: {}", val);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    }
//println!(" result {} ", v2);
//println!("  {:#?} ",iterator_sum);

}
/* https://doc.rust-lang.org/book/ch13-02-iterators.html
 */
 

