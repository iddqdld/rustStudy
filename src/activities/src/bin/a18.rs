// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`

#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
        if age >= 21 {
            Ok(Self{
                age,
                name: name.to_string(),
            })
        } else {
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
            Err("Age must be at least 21")
        }
    }
}

// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

fn main() {
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
    let child = Adult::new(16, "Alice");
//   * One should be 21 or over
    let adult = Adult::new(22, "Maria");

    match child {
        Ok(c) => println!("Age : {:?} Name : {:?}", c.age, c.name),
        Err(e) => println!("{e}"),
    }

    match adult {
        Ok(a) => println!("Age : {:?} Name : {:?}", a.age, a.name),
        Err(e) => println!("{e}")
    }
}
