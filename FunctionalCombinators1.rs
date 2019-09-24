
fn main(){
let vector = vec!(1,3,4,5,3);
let max = vector.iter().max().unwrap();
println!("{}",max);
let sum = vector.iter().fold(0,|sum,value| sum+value);
println!("{}",sum);
}
