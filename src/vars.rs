//Variables hold prinitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name = "Josh";
    //use mut keyword in front of variable name to make the variable mutable
    let mut age = 32;
    println!("My name is {} and I am {}", name, age);

    age = 25;

    println!("My name is {} and I am {}", name, age);

    //Define constants
    //All uppercase, must define a type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let ( my_name, my_age) = ("Josh", 32);
    println!("{}", my_name);
    println!("{}", my_age);
}