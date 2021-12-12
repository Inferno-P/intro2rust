// Variable Info:
// 1. Variable only store primitive data or refferences to data.
// 2. Variables are immutable by default i.e. You cant re-assign them by default.
// 3. Rust is a block scoped language i.e. x defined inside a func foo will pertain to the scope(or block) of foo only.

pub fn run() {
    let name = "Williams";
    let mut age = 32; // Adding the keyword 'mut' before the var intialization makes it mutable 
    // i.e. we can change it's value.

    println!("My name is {} and I am {}.", name, age) ;
    age = age+1; // Williams grew up! 
    println!("My name is {} and I am {}.", name, age) ;

    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assing multiple  vars
    let (myname, myage) = ("Alice", 37);
    println!("My  name is {0}. I am {1} years old.", myname, myage);

}