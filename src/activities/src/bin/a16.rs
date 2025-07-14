// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let student_a = Student {
        name: "Alex".to_owned(),
        locker: Some(13),
    };
    println!("Student in question is {:?}", student_a.name);
    match student_a.locker {
        Some(ans) => println!("Student's locker is {:?}", ans),
        None => println!("Student didn't has a locker"),
    }
}
