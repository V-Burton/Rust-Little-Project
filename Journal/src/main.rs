mod cli;
mod task;

use anyhow::anyhow;
use structopt::StructOpt;
use cli::{Action, CommandLineArgs};
use task::Task;
use std::path::PathBuf;
use std::io::{self, Write};
use colored::*;

fn print_menu() {
    println!("Choose an action:");
    println!("1. Add a task");
    println!("2. List tasks");
    println!("3. Mark task as done");
    println!("Type 'quit' to quit the program.");
    print!("> ");
    io::stdout().flush().unwrap();
}

fn parse_action(input: &str) -> Option<Action> {
    match input {
        "1" => {
            print!("Enter the task to save: ");
            io::stdout().flush().unwrap();
            let mut task = String::new();
            io::stdin().read_line(&mut task).unwrap();
            let task = task.trim().to_string();
            Some(Action::Add {task})
        },
        "2" => Some(Action::List),
        "3" => {
            print!("Enter the task number to mark as done: ");
            io::stdout().flush().unwrap();
            let mut position = String::new();
            io::stdin().read_line(&mut position).unwrap();
            if let Ok(position) = position.trim().parse::<usize>() {
                Some(Action::Done {position})
            } else {
                None
            }
        },
        _ => None,
    }
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rust-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()>{
    
    let CommandLineArgs {
        journal_file,
        ..
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Failed to find journal file."))?;

    loop {
        print_menu();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.eq_ignore_ascii_case("quit"){
            break;
        }
        match parse_action(input) {
            Some(Action::Add {task}) => {
                if let Err(e) = task::add_task(&journal_file, Task::new(task)) {
                    println!("Error: {}", e.to_string().red());
                }
            },
            Some(Action::List) => { 
                if let Err(e) = task::list_tasks(&journal_file) {
                    println!("Error: {}", e.to_string().red());
                }

            },
            Some(Action::Done {position}) => {
                if let Err(e) = task::complete_task(&journal_file, position) {
                    println!("Error: {}", e.to_string().red());
                }
            },
            None => println!("Invalid action. Please try again."),
        }
    }

    Ok(())
}
