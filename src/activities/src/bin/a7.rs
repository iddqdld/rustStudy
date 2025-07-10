// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors{
    Red,
    Green,
    Blue
}

fn color_name(object: Colors){
    match object {
        Colors::Red => println!("this is red"),
        Colors::Green => println!("this is green"),
        Colors::Blue => println!("this is blue"),
    }
}

fn main() {
    let ball = Colors::Red;
    let stick = Colors::Blue;
    let cube = Colors::Green;
    color_name(ball);
    color_name(stick);
    color_name(cube);
}
