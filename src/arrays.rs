//Arrays -Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    //Re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);

    //Get single val
    println!("Single Value: {}", numbers[2]);

    //Get array length
    println!("Array Length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes.", mem::size_of_val(&numbers));

    //Get slice
    // Using & indicates sending a reference pointer 
    let slice: &[i32] = &numbers[1..2];
    //Must use {:?} for printing debug options
    println!("Slice: {:?}", slice);

}