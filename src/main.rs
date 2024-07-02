use std::io;

fn main() {
    let mut todos: Vec<String> = Vec::new();

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
            1 => create_todo(&mut todos), // create
            2 => read_todo(&mut todos), // read
            3 => update_todo(&mut todos), // update
            4 => {

            }
            5 => {

            }
            6 => {
                println!("Goodbye 👋");
                break;
            }
            _ => {} // This shouldn't run
        };
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

fn create_todo(todos: &mut Vec<String>) {
    let mut input = String::new();

    println!("What is your new todo?");

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    todos.push(input);
}

fn read_todo(todos: &mut Vec<String>) {
    if todos.len() == 0 {
        println!("\nYou have no todos. Please make one.\n");
        return;
    }

    let mut input = String::new();

    println!("\nEnter the number of your todo that you wish to read.");

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    match input.trim().parse::<usize>() {
        Ok(number) => {
            if (number - 1) > todos.len() {
                println!("\nThis todo does not exist.\n");
                return;
            }

            println!("\nHere is todo number {}:", number - 1);
            println!("{}", todos[number - 1]);
        },
        Err(_) => {
            println!("\nAn error occured searching for your todo. Please try again\n");
            return;
        }
    };
}

fn update_todo(todos: &mut Vec<String>) {
    if todos.len() == 0 {
        println!("\nYou have no todos. Please make one.\n");
        return;
    }

    println!("Enter the number of the todo you wish to edit.");
    
    let mut todo_input = String::new();

    io::stdin().read_line(&mut todo_input).expect("Failed to read line.");

    let index = match todo_input.trim().parse::<usize>() {
        Ok(number) => {
            if (number - 1) > todos.len() {
                println!("\nThis todo does not exist.\n");
                return;
            }

            number - 1
        },
        Err(_) => {
            println!("\nAn error occured searching for your todo. Please try again\n");
            return;
        }
    };

    let mut new_todo_input = String::new();

    println!("Enter your updated todo:");

    io::stdin().read_line(&mut new_todo_input).expect("Failed to read line.");

    println!("Updating todo number {}", index + 1);
    println!("Previous: {}", todos[index]);
    println!("New: {}", new_todo_input);

    todos[index] = new_todo_input;
}