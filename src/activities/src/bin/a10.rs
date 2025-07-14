// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100

// * Use a function to print the messages

// * Use a match expression to determine which message
//   to print

fn display_result(result: bool) {
    match result {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let var = 101;
    let result = if var > 100 {    // let result = var > 100; also works and should be iplemented asa a shortcut, but note says we shoud've used an if expression.
        true
    } else {
        false
    };

    display_result(result);
}