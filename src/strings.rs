// Primitive str = Immmutable fixed-length string somewhere in memory.
// String = Growable, heap-allocated data structure  Use when one needs to modify the string.

pub fn run(){

    let _hello = "Hello!"; // Primitive type string

    let mut hello = String::from("Hello"); // Growable type string.


    //Get length 
    println!("Length: {}", _hello.len());

    //Add a character
    hello.push('W'); // Add a character. The string var has to be mutable for this to work.
    println!("String = {}.", hello);


    // Add a string
    hello.push_str("orld!");
    println!("String = {}.", hello);

    // Get mem capacity of a string var(in Bytes)
    println!("Capacity = {}.\nLength = {}.", hello.capacity(),hello.len());


}