//rustc --explain E0277


// we now declare a function which takes an object implementing the Foo trait
/*fn some_func<T: Foo>(foo: T) {
    foo.bar();
}
*/
/*fn main() {
    // we now call the method with the i32 type, which doesn't implement
    // the Foo trait
    some_func(5i32); // error: the trait bound `i32 : Foo` is not satisfied
}
*/

/*In order to fix this error, verify that the type you're using does implement
the trait. Example:
*/

trait Foo {
    fn bar(&self);
}

fn some_func<T: Foo>(foo: T) {
    foo.bar(); // we can now use this method since i32 implements the
               // Foo trait
}

// we implement the trait on the i32 type
impl Foo for i32 {
    fn bar(&self) {}
}

fn main() {
    some_func(5i32); // ok!
}
