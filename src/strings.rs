/* Primitive String = Immutable foxed-length string somewhere in memory
    String  = Growable, heal-allocated data structure - Use when yoou need to modify or own string data
*/


pub fn run(){
    //Need to use String library
    let mut hello = String::from("Hello ");

    //Push single char (single quotes)
    hello.push('W');
    //Push whole string (double quotes)
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());
    //Check if empty
    println!("is Empty?: {}", hello.is_empty());
    //Contains
    println!("Contains 'World'?: {}", hello.contains("World"));
    //Replace
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    println!("{}", s);

    //Assertions testing - only shows error if fails
    assert_eq!(2, s.len());
    assert_eq!(12, s.capacity());
    
    println!("{}", hello);
    
    //Get length
    println!("Length: {} ", hello.len());

}