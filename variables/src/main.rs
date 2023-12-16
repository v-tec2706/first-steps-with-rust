fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; error: cannot reassing to wariable
    
    let mut y = 4;
    y = 5;

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; // must have type declaration

    // shadowing
    let x = 1;
    let x = 2; // first `x` is shadowed, that is fine

    let numbers = [1,2,3,4]; // array are used for fixed size collections
    
    let a = [3; 5];
    let b = [3,3,3,3,3];
    // a == b, they are quivalent

    fn plus_one_not_ok(x: i32) -> i32 {
        x + 1; // it's not valid as adding `;` makes it a statement rather than expression
    }

    fn plus_one_ok(x: i32) -> i32 {
        x + 1
    }

    // expression (return value) vs statement (doesn't returns `()`)
    
    
    let condition = true:
    let number = if condition { 5 } else { 6 };

    // loop labels
    'conting_up: loop {
        println!("count loop")

        let random_number = 3;
        
        loop {
            if random_number > 0 {
                break;
            }
            if random_number < 0 {
                break 'conting_up;
            }
        }
    }

    // for loop 
    fn main() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
