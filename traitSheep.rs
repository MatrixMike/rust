//https://doc.rust-lang.org/rust-by-example/trait.html

struct Sheep  { naked: bool, name: &'static str }
struct Rabbit { naked: bool, name: &'static str }

trait Animal {
    // Static method signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Instance method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
			println!("{} gets a haircut!", self.name);
			 self.naked = true;
        }
    }
        fn fleece_grow(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} fleece has grown...", self.name());
        } else {
			println!("{} gets a haircut!", self.name);
			 self.naked = false;
        }
    }
}

impl Rabbit {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn stroke(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah? cold"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
impl Animal for Rabbit {
    // `Self` is the implementor type: `Rabbit`.
    fn new(name: &'static str) -> Rabbit {
        Rabbit { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah? cold"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}
fn main() {
	// Type annotation is necessary in this case. 
	let mut dolly: Sheep  = Animal::new("Dolly"); 
	let mut bunny: Rabbit = Animal::new("Bunny"); 
	// TODO ^ Try removing the type annotations.
    if dolly.is_naked() {
		println!("true"); } else {
			println!("false");
		}
	dolly.talk(); dolly.shear(); dolly.shear(); dolly.shear(); dolly.fleece_grow(); dolly.talk(); 
	bunny.talk(); bunny.stroke();
	println!("{}",dolly.name());
	println!("{}",bunny.name());
}
