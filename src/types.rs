/*Primitive Types --
    Integers: u8, i8, u16, u32, u64, u128, i128 (number of bits they take in memory)
    Floats: f32, f64
    Boolean: (bool)
    Tuples
    Arrays
    Characters (char)
*/

//Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how weuse it.
pub fn run(){
    //Default is i32
    let x = 1; 
    //Default is f64
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545454545;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean - Implicit
    // let is_active = true;

    //Boolean - Explicit
    let is_active: bool = true;

    //Get Boolean from expresion
    let is_greater: bool = 10 > 5;

    //Char signified with single quotes
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face))
}