#![allow(unused)]
fn main() {
    (0..5).flat_map(|x| x * 100 .. x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0)
        .for_each(|(i, x)| println!("{}:{}", i, x));
    

    (0..10)
        .flat_map(|x| x * 100..x * 110)
        .enumerate()
        .filter(|&(i, x)| (i + x) % 3 == 0) // first invocation
        .filter(|&(i, x)| (i + x) % 5 == 0) // copied above and changed - think fizz,buzz
        .for_each(|(i, x)| println!("{:3} xx {}", i, x));  
        // println!("{:a3} xx {}", i, x) enable this for format help
    //     println!("asd");
    let lines_vec = vec!["hello,how", "are,you"];
    let words_vec = lines_vec
        .iter()
        .flat_map(|&x| x.split(","))
        .collect::<Vec<&str>>();
    println!("{:?}", words_vec);
}
