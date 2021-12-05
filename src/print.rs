pub fn run() {
    //Print to console
    println!("Hello from the print file."); 

    println!("{} is {} years old.", "Alice", "21");


    println!("{0} is from {1} and {0} likes to sing.","Alice","UK");


    println!("{name} is good at {sport}.", name="Jose", sport="Boxing");


    //Placeholder traits 
    println!("Binary : {:b}. Hex: {:x}, Octal: {:o}", 10,10,10);


    //Placeholder for debug trait
    println!("{:?}", (12, true,"hello"));


    //Basic math
    println!("10 + 10 = {ans}", ans=(10+10)); 

}

