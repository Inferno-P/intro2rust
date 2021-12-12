// Tuples group together values of different types
// Maximum 12 elements only. 


pub fn run(){

    let person: (&str, &str, i8) = ("Adam","ok", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);



}