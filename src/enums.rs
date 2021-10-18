//Enums are types which have a few definite values

enum Movement{  
    //Variants
    Up,
    Down, 
    Left, 
    Right
}

fn move_avatar(m: Movement) {
    //perform action pending on info
    match m {
        Movement::Up => println!("AVatar moving up...", ),
        Movement::Down => println!("AVatar moving down...", ),
        Movement::Left => println!("AVatar moving left...", ),
        Movement::Right => println!("AVatar moving right...", )
    }

}

pub fn run() {

    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

}