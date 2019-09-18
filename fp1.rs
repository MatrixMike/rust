fn main() {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let fn_variable = add;
    println!("calling using function variable {}", fn_variable(10, 20));
}
