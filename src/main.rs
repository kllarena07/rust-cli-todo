use std::io;

fn main() {
    loop {
        print_all_options();
    }
}

fn print_all_options() {
    println!("What would you like to do?");
    println!("1. Create a todo");
    println!("2. Read a todo");
    println!("3. Update a todo");
    println!("4. Delete a todo");
    println!("5. List all todos");
    println!("6. Exit");
}
