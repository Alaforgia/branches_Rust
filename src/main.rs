



// Conditional loops w/ while
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");
}

// Returning values from loops
// fn loops() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//         // the loop will break when the counter hits number 20.
//     };
//     println!("The result is {}", result);
// }


// // if in a let statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("The value of number is: {}", number);

// }

// else/if
// fn main() {
// Rust will find the first true block and not look for the rest.
//     let number = 6;
//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }
//
//
// if/else conditions
//
// fn main() {
//     let number = 3;

//     if number != 0  {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }
