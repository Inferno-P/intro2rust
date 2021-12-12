// Vectors - Re-sizeable arrays. 


pub fn run(){

    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("{:?}", numbers); // Debug mode 

    numbers[1] = 20;


    //Get single value
    println!("Single Value @ 1 = {}", numbers[0]);


    //Get vector length
    println!("Vector length = {}", numbers.len());


    //Arrays are stack allocated
    println!("Vector occupies {} bytes.", std::mem::size_of_val(&numbers));

    // hello asdasd

    //Get slice 
    let slice: &[i32] = &numbers[0..2];

    println!("Slice = {:?}",slice);

    //Add on the vector 
    numbers.push(5);
    numbers.push(6);

    println!("After pushing numbers, vector = {:?}", numbers);


    //Pop
    numbers.pop();
    println!("AFter popping:{:?}", numbers);

    //Looping 

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop and mutate values 
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers : {:?}", numbers );

}