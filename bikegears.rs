use std::env;
// 17.09.2019 16:07:26
fn main() {
let frontGearVector = vec!(25,35, 52);	
let  rearGearVector = vec!(12,14,16,18,22,24,30);		
let newVector = rearGearVector.iter().map(|&x| x +1).collect::<Vec<i32>>();
 println!("{:?}", rearGearVector);
 println!("{:?}", newVector); 
}
