use std::io;

fn main() {
    let mut todos: Vec<String> = vec![];

    loop {
        print_all_options();
        
        let selected_option = match get_option() {
            Ok(option) => option,
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        match selected_option {
            1 => {

            }
            2 => {
                
            }
            3 => {

            }
            4 => {

            }
            5 => {

            }
            6 => {
                println!("Goodbye 👋");
                break;
            }
            _ => {} // This shouldn't run
        }
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

fn get_option() -> Result<u8, String> {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    match input.trim().parse::<u8>() {
        Ok(number) if number >= 1 && number <= 6 => {
            Ok(number)
        }
        _ => {
            Err(String::from("\nInvalid option. Please try again.\n"))
        }
    }
}