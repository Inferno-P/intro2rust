// Fixed length and same data types.


pub fn run(){

    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("{:?}", numbers); // Debug mode 

    numbers[1] = 20;


    //Get single value
    println!("Single Value @ 1 = {}", numbers[0]);


    //Get array length
    println!("LEngth of array = {}", numbers.len());


    //Arrays are stack allocated
    println!("Array occupies {} bytes.", std::mem::size_of_val(&numbers));

    // hello asdasd

}