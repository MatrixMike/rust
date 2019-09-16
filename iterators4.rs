#![allow(unused)]
fn main() {
/*(0..5).flat_map(|x| x * 100 .. x * 110)
      .enumerate()
      .filter(|&(i, x)| (i + x) % 3 == 0)
      .for_each(|(i, x)| println!("{}:{}", i, x));
  */  
      (0..10).flat_map(|x| x * 100 .. x * 110)
      .enumerate().filter(|&(i, x)| (i + x) % 3 == 0)
      .for_each(|(i, x)| println!("{} xx {}",  i, x));
 //     println!("asd");
}
