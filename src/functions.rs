
//Functions are used to store a block of code for later use

pub fn run(){
    greeting("Hello", "Josh");

    //Bind function values to variables
    let get_sum = add(5, 15);
    println!("Sum: {}", get_sum);

    //Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3))
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you!", greet, name);
}
//type of data needs to be declared in the function statement
// -> is used to return a value. No semicolons used in a return block
fn add(n1: i32, n2:i32) -> i32 {
    n1 + n2
}