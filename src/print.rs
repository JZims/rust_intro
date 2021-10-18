
pub fn run() {
    //Print to console
    println!("Hello from the print.rs file!");

    //Basic Formatting
    println!("{} is from {}", "Josh", "FL");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}.", "Josh", "FL", "chill");

    //Named Arguments
    println!("{name} likes to play {activity}.", name = "Josh", activity = "Magic");

    //Placeholder Traits
    println!("Binary: {:b}  Hex: {:x}  Octal: {:o}", 10, 10, 10 );

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}