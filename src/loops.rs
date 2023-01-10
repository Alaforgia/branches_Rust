
// Returning values from loops
fn loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        // the loop will break when the counter hits number 20.
    };
    println!("The result is {}", result);
}
