// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Colors {
    Red,
    Green,
    Blue,
}

impl Colors {
    fn print(&self) {
        match self {
            Colors::Red => println!("The color is red"),
            Colors::Green => println!("The color is green"),
            Colors::Blue => println!("The color is blue"),
        }
    }
}

struct Dimensions {
    width: f64,
    heigth: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("The width is {:?}", self.width);
        println!("The height is {:?}", self.heigth);
        println!("The depth is {:?}", self.depth);
    }
}

struct BoxStats {
    dim: Dimensions,
    weight: f64,
    color: Colors
}

impl BoxStats {
    fn new(dim: Dimensions, weight: f64, color: Colors) -> Self {
        Self {
            dim,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dim.print();
        println!("The weight is {:?}", self.weight);
    }
}

fn main() {
    let small_dimensions = Dimensions {
        width: 1.0,
        heigth: 2.0,
        depth: 1.5,
    };
    let small_box = BoxStats::new(small_dimensions, 5.0, Colors::Red);
    small_box.print();
}
