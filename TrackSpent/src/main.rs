use std::io;
use std::collections::HashMap;
use std::fs;
use std::collections::VecDeque;

mod display;
mod spent;

use spent::{push_transaction_to_result, Spent, TransactionsData};
use display::{display_choice, display_transaction, show_categories};

fn sort(result: &mut VecDeque<spent::Spent>, outcome: &mut HashMap<String, Vec<Spent>>, income: &mut HashMap<String, Vec<Spent>>) {
   while let Some(item) = result.pop_front() {
        display_transaction(&item);
        let mut input = String::new();
        if item.amount > 0.0 {
            let keys = income.keys().collect::<Vec<&String>>();
            display_choice(&keys);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<usize>() {
                Ok(choice) if choice > 0 && choice <= keys.len() => {
                    let key = keys[choice - 1].clone();
                    income.get_mut(&key).unwrap().push(item);
                },
                Ok(choice) if choice == keys.len() + 1 => {
                    add_key(income);
                    result.push_front(item);
                },
                Ok(choice) if choice == 0 => {
                    result.push_front(item); break
                },
                Ok(_) | Err(_) => {
                    println!("Invalid choice. Please try again.");
                    result.push_front(item);
                },
            }
        } else {
            let keys = outcome.keys().collect::<Vec<&String>>();
            display_choice(&keys);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim().parse::<usize>() {
                Ok(choice) if choice > 0 && choice <= keys.len() => {
                    let key = keys[choice - 1].clone();
                    outcome.get_mut(&key).unwrap().push(item);
                },
                Ok(choice) if choice == keys.len() + 1 => {
                    add_key(outcome);
                    result.push_front(item);
                },
                Ok(choice) if choice == 0 => {
                    result.push_front(item); break
                },
                Ok(_) | Err(_) => {
                    println!("Invalid choice. Please try again.");
                    result.push_front(item);
                },
           }
       }
    }
    if result.is_empty() {
        println!("All value have been sorted!");
    } else {
        println!("There are still {} transactions to sort", result.len());
    }
}

fn intialize(outcome: &mut HashMap<String, Vec<Spent>>, income: &mut HashMap<String, Vec<Spent>>) {
    outcome.insert("Charges".to_string(), Vec::new());
    outcome.insert("Food".to_string(), Vec::new());
    outcome.insert("Save".to_string(), Vec::new());
    outcome.insert("Other".to_string(), Vec::new());
    income.insert("Revenu".to_string(), Vec::new());
    income.insert("Refund".to_string(), Vec::new());
    income.insert("Gift".to_string(), Vec::new());
    income.insert("Other".to_string(), Vec::new());
}

fn add_key(map: &mut HashMap<String, Vec<Spent>>) {
    let mut input = String::new();
    println!{"Please enter the name of the new category: "};
    io::stdin().read_line(&mut input).expect("Failed to read line");
    map.insert(input.trim().to_string(), Vec::new());
}

fn main() {
    let mut outcome: HashMap<String, Vec<Spent>> = HashMap::new();
    let mut income: HashMap<String, Vec<Spent>> = HashMap::new();

    intialize(&mut outcome, &mut income);

    let file_content = fs::read_to_string("checkData.json").expect("Unable to read file");
    let mut data: TransactionsData = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    
    let mut result: VecDeque<spent::Spent> = VecDeque::new();
    
    push_transaction_to_result(&mut result, &mut data);

    loop {
        println!("What do you want to do?\n\t Sort \n\t Show categories\n\t quit");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "Sort" => sort(&mut result, &mut outcome, &mut income),
            "Show categories" => show_categories(&outcome, &income),
            "quit" => break,
            _ => continue,
        }

    }
}

