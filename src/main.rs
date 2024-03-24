use std::io;
use std::process::exit;

struct TodoItem {
    todo_value: String,
    done: bool,
}

const NUMBER_OF_ACTIONS: i32 = 5;

fn main() {
    let mut todo_list: Vec<TodoItem> = vec![];

    loop {
        println!("\nWhat do you want to do?");
        println!("1.Get the list of all todos");
        println!("2.Add a todo");
        println!("3.Remove a todo");
        println!("4.Mark a todo done");
        println!("5.Exit");

        //Convert action_input which is string to i8
        let input: Option<i32> = convert_to_num(&get_input().trim().to_string());
        //Clears the console
        clearscreen::clear().expect("failed to clear screen");

        match input {
            Some(num) => match num {
                1 => get_all_todos(&todo_list),
                2 => add_todo(&mut todo_list),
                3 => remove_todo(&mut todo_list),
                4 => mark_done(&mut todo_list),
                5 => exit(0),
                _ => {}
            },
            None => {
                continue;
            }
        }
    }
}

fn convert_to_num(input: &String) -> Option<i32> {
    match input.parse::<i32>() {
        // Checks if input is convertable and if it's equal to 4
        Ok(num) if num <= NUMBER_OF_ACTIONS => Some(num),

        // Matches if there is error of converting
        Err(err) => {
            eprintln!("ERROR: {err}");
            None
        }

        // Matches if entered number is bigger than choise list
        _ => {
            println!("Choose between 1, 2, 3 and 4!");
            None
        }
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Could not get input");
    s = s.trim().to_string();
    s
}

fn get_all_todos(todo_list: &Vec<TodoItem>) {
    if todo_list.len() != 0 {
        for (i, todo) in todo_list.iter().enumerate() {
            let prefix: String = {
                if todo.done {
                    "x".to_string()
                } else {
                    "-".to_string()
                }
            };

            println!(
                "{index}. [{prefix}] {value}",
                index = i + 1,
                value = todo.todo_value
            );
        }
    } else {
        println!("Your list is empty! Add a todo to get started");
    }
}

fn add_todo(todo_list: &mut Vec<TodoItem>) {
    println!("What are you plannig todo?:");
    let todo_value = get_input();

    if todo_value != "" {
        let new_todo = TodoItem {
            todo_value,
            done: false,
        };
        todo_list.push(new_todo);
    } else {
        println!("Todo can't be an empty string!");
    }
}

fn remove_todo(todo_list: &mut Vec<TodoItem>) {
    get_all_todos(todo_list);
    println!("Choose the todo you want to delete:");
    let temp: String = get_input();
    let index = match temp.parse::<i32>() {
        // indexes provided to user are 1 index ahead of the actual index
        Ok(num) => Some(num - 1),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            None
        }
    };

    if let Some(index) = index {
        if (index as usize) < todo_list.len() {
            println!("Are you sure?: [Y]es, [N]o");
            let choice: String = get_input();
            match &choice[..] {
                "Y" => {
                    todo_list.remove(index as usize);
                    println!("Deleted");
                }
                _ => {
                    println!("Skipping...");
                }
            }
        } else {
            eprintln!("Index out of range")
        }
    }
}

fn mark_done(todo_list: &mut Vec<TodoItem>) {
    get_all_todos(todo_list);
    println!("Choose the todo you want to mark as done:");
    let temp: String = get_input();
    let index = match temp.parse::<i32>() {
        // indexes provided to user are starting from 1, so have to subtract 1 from to in order to make it compatible with vector indexes
        Ok(num) => Some(num - 1),
        Err(err) => {
            eprintln!("ERROR: {}", err);
            None
        }
    };

    match index {
        Some(index) => {
            todo_list[index as usize].done = true;
            println!("Markerd as done");
        }
        None => (),
    }
}
