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
    hello.push_str("orld  ol' billy !");
    println!("String = {}.", hello);

    // Get mem capacity of a string var(in Bytes)
    println!("Capacity = {}.\nLength = {}.", hello.capacity(),hello.len());
    
    //Check if substrsing present? 
    println!("Contains 'World' {} ", hello.contains("World"));

    //Replace string 
    println!("Replace: {}", hello.replace("World"," There ol billy!"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}",word);
    }
    
    // Create string with capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}",s);

    //Assertion testing 
    assert_eq!(2, s.len());//No response if the stmt is correct. In case of false stmt, it'll terminate the program.

    println!("{}", s)




}