//use std::env;
// 17.09.2019 16:07:26
// camel case should be changed for snake case
fn main() {
let _front_gear_vector = vec!(25,35, 52);	
let  _rear_gear_vector = vec!(12,14,16,18,22,24,30);		
let new_vector = _rear_gear_vector.iter().map(|&x| x +1).collect::<Vec<i32>>();
 println!("{:?}", _rear_gear_vector);
 println!("{:?}", new_vector); 
}