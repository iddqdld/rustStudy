// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn display_quantity(item: &Grocery){
    println!("quantity = {:?}", item.quantity);
}

fn display_id(item: &Grocery){
    println!("id = {:?}", item.id);
}

fn main() {
    let milk = Grocery {
        quantity: 5,
        id: 1,
    };
    display_quantity(&milk);
    display_id(&milk);
}
