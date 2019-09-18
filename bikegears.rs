//use std::env;
// 17.09.2019 16:07:26
// camel case should be changed for snake case
fn main() {
    let _vec_of_float = vec![2.0, 3.0];
    let _front_gear_vector = vec![25.0, 35.0, 52.0];
    let _rear_gear_vector = vec![12.0, 14.0, 16.0, 18.0, 22.0, 24.0, 30.0];
    let mut unsorted_integers = vec![9, 5, 7, 2, 1, 11];
    let new_vector = _rear_gear_vector
        .iter()
        .map(|&x| x / 2.0)
        .collect::<Vec<f32>>();
       unsorted_integers.sort() ;
    let sorted_integers = unsorted_integers;
    println!("{:?}", sorted_integers);
    println!("{:?}", _rear_gear_vector);
    println!("{:?}", new_vector);
}
