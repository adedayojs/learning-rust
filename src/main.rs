/* fn main() {
    let x = 45;
    println!("x is this value you are about to see: {}", x);
    let x = "ade";
    println!("x is this value you are about to see: {}", x);

    //  DATA TYPES

    let a: f32 = 10.34; // Floating point value using 32 bits (Single precision)
    let b: f64 = 10.34; // Floating point value using 64 bits (Double precision)

    let ade: char = 'a';

    let ff = "hggh";


}
 */
/*
use std::io;

fn main() {
    let mut first_name_input = String::new();
    let mut last_name_input = String::new();

    println!("What is your First name");
    io::stdin().read_line(&mut first_name_input).expect("Failed to read line");

    println!("What is your Last name");
    io::stdin().read_line(&mut last_name_input).expect("Failed to read line");

    println!("Your name is: {} {} ", first_name_input.trim(), last_name_input);
} */

/* fn main() {
    let food = "Cookies";
    if (food == "Cookies") {
        print!("I Love cookies too")
    }
} */

// fn main() {
//     // This would not work
//     // let x:i32 = 20;
//     // let y:i32 = 30;

//     let x = 20;
//     let y = 30;

//     add_numbers(x, y);
// }

// fn add_numbers(x: i64, y: i64) {
//     println!("The sum of the numbers you entered is {} ", x + y)
// }

/***
 *
 *
 * GUESSING GAME
 */

/* use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
 */

/**
 *
 * COMMON PROGRAMMING CONCEPTS
 */

fn main() {
    fn accessing_invalid_index() {
        let a = [1, 2, 3, 4, 5];
        let index = 10;
        let element = a[index];
        println!("The value of element is: {}", element);
    }

    /*   fn testing_scope() {
        let x = 32;

        fn another_scope() {
            println!("If this uses the outer scope like is suggest x should be 32...Let's see..... X is {}", x);

            let x = { x + 32 };
            println!("If I'm correct, the Value of X should have changed as i have shadowed the x variable from the outer scope.... X is Now {}", x)
        }
        // Okay so apparently i cannot access variables from a parent scope as i would have done in JavaScript... Let's continue learning
    } */

    fn testing_scope_2() {
        let x = 32;

        println!("X is {}", x);

        let y = {
            let x = x + 32;
            println!("This should be the value of our new x which is adding 10 to our previous x... X is Now {}", x);
            x
        };

        println!("X in this scope shoud not have changed... X is {}", x)
    }

    fn testing_return_type_of_function() {
        fn five() -> i64 {
            5
        }

        let five = five();
        println!("Five is {}", five)
    }

    fn is_string_literal_immutable() {
        let mut string_literal = "Bobby";
        string_literal = &(string_literal.to_owned() + "bob");
        println!("{}", string_literal)
    }
    // accessing_invalid_index();
    // testing_scope()
    // testing_scope_2()
    testing_return_type_of_function()
}
