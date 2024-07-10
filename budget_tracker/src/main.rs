mod cli;
mod task;

use cli::{Action};
use task::{Budget, Category, Spent};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() {
    let budget_file = find_default_budget_file().ok_or("Failed to load file.");

    loop {
        print_menu(){
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;;
            let input = input.trim();
            if input.eq_ignore_ascii_case("quit"){
                break;
            }

            match parse_action(input) {
                Some(Action::Add) => {
                    if let Err(e) = task::add_spent(&budget_file, Task::new(task)) {
                        println!("Error: {}", e.to_string().red());
                    }
                },
                Some(Action::Display) => { 
                    if let Err(e) = task::list_tasks(&budget_file) {
                        println!("Error: {}", e.to_string().red());
                    }

                },
                None => println!("Invalid action. Please try again."),
            }    
        }
    }
}
