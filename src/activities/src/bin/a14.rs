// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    age: i32,
    name: String,
    color: String,
}

fn print(data: &str){
    println!("{:?}", data);
}

fn main() {
    let class_a1 = vec! [
        Person {
            age: 9,
            name: "Kevin".to_owned(),
            color: "red".to_owned(),
        },
        Person {
            age: 11,
            name: "Peter".to_owned(),
            color: "green".to_owned(),
        },
        Person {
            age: 7,
            name: "Brad".to_owned(),
            color: "blue".to_owned(),
        },
    ];

    for kid in class_a1 {
        if kid.age <= 10 {
            print(&kid.name);
            print(&kid.color);
        }
    }
}
