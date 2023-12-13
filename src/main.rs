use std::env;
use todo_bin::{help, Todo};


//set TODO_BAK_DIR="./backup";

fn main() {
    let todo = Todo::new().expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..], "user"),
            "rm" => todo.remove(&args[2..]),
            "done" => todo.done(&args[2..]),
            "raw" => todo.raw(&args[2..]),
            "sort" => todo.sort(),
            "clear" =>todo.clear(),
            "help" | "--help" | "-h" | _ => help(),
        }
    } else {
        todo.list();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

   #[test]
    fn test_add() {
        // Create a new Todo instance or panic if there's an error
        let todo_result = Todo::new();
        let mut todo = todo_result.expect("Failed to create Todo instance");

        // Clear any existing tasks
        todo.clear();

        // Add tasks
        todo.add(&vec!["Task1".to_string(), "Task2".to_string()], "user");

        // Read the contents of the todo file
        let contents = fs::read_to_string("todo.txt").expect("Failed to read todo file");
        println!("{}",contents);

        // Expect the final state of the todo file
        assert_eq!(contents, "[ ] Task1\n[ ] Task2\n");
        todo.clear();

        // Clean up: remove the temporary todo file
        fs::remove_file("todo.txt").expect("Failed to remove todo file");
    }

}