/*
Primitive Types : 
1. Integers : u8 - u128, i8 - i128;(u=unsigned,i=signed)
2. Floats : f32, f64
3. Boolean : bool
4. Characters : char
5. Tuples
6. Arrays 

Note : Rust is a statically/strictly typed language, which means that it must know the types of all
variables defined at compile time, however, the compiler can usually infer what type we want to use 
based on the value and how we want to use it.

*/ 

pub fn run() {
    let x = 1; //Default is i32.
    let y = 2.5; // Default is f32.
    let z: i64 = 4212431290; 



//Find max size
println!("Max i32: {}", std::i32::MAX);

let is_active: bool = true;

let is_greater:bool = 10 < 5; 


let a1 = 'a';
let face = '\u{1F600}';

println!("{:?}",(x,y,z, is_active, is_greater, face, a1));


    

    
}