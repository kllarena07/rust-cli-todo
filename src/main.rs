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
            1 => create_todo(&mut todos),
            2 => read_todo(&mut todos),
            3 => update_todo(&mut todos),
            4 => delete_todo(&mut todos),
            5 => print_all_todos(&mut todos),
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

    println!("\nWhat is your new todo?");

    io::stdin().read_line(&mut input).expect("Failed to read line.");

    println!("\nSuccessfully created new todo: {}", input);

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

            println!("\nHere is todo number {}:", number);
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

    println!("\nEnter the number of the todo you wish to edit.");
    
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

    println!("\nEnter your a new value:");

    io::stdin().read_line(&mut new_todo_input).expect("Failed to read line.");

    println!("\nUpdating todo number {}", index + 1);
    print!("Previous: {}", todos[index]);
    println!("New: {}", new_todo_input);

    todos[index] = new_todo_input;
}

fn delete_todo(todos: &mut Vec<String>) {
    if todos.len() == 0 {
        println!("\nYou have no todos. Please make one.\n");
        return;
    }

    println!("\nEnter the number of the todo you want to delete.");

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

    todos.remove(index);

    println!("\nSuccessfully deleted todo {}\n", index + 1);
}

fn print_all_todos(todos: &mut Vec<String>) {
    if todos.len() == 0 {
        println!("\nYou have no todos. Please make one.\n");
        return;
    }

    println!("\nHere are all your todos:");
    for i in 0..todos.len() {
        print!("{}. {}", i + 1, todos[i]);
    }
    println!("");
}