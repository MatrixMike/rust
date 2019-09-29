fn main (){
	   let array = [1u32, 3, 3, 7,2,6,9,4,10];

    // The `iter` method produces an `Iterator` over an array/slice.
    println!("Iterate the following array {:?}", &array);
    for i in array.iter().skip(4).take(4) {
        println!("> {}", i*i);
    }
    for i in 3..10 {
		println!("> {}", i*i);
	}
	for i in (3..10).skip(4) {
		println!("> {} {}",i, i*i);
	}
}
