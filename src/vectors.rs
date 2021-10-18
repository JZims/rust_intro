
//Vectors - Resizable arrays

use std::mem;

pub fn run() {
    //Assigned with capital Vec<data_size> and set with vec![etc..];
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-assign a value
    numbers[2] = 20;

    

    //Add onto Vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);

    //Pop off last value
    numbers.pop();

    println!("{:?}", numbers);
    
    //Get single val
    println!("Single Value: {}", numbers[2]);

    //Get Vector length
    println!("Vector Length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes.", mem::size_of_val(&numbers));

    //Get slice
    // Using & indicates sending a reference pointer 
    let slice: &[i32] = &numbers[1..2];
    //Must use {:?} for printing debug options
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut(){
        *x *= 2
    }

    println!("numbers Vec: {:?}", numbers);

}