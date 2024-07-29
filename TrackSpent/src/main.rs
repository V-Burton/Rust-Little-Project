use std::io;
use std::collections::HashMap;
use std::fs;
use std::collections::VecDeque;

mod display;
mod spent;

use spent::{push_transaction_to_result, Spent, Transaction, TransactionsData};
use display::{display_choice, display_transaction, show_categories};

fn sort(result: &mut VecDeque<spent::Spent>, outcome: &mut HashMap<String, Vec<Spent>>, income: &mut HashMap<String, Vec<Spent>>) {
   while let Some(item) = result.pop_front() {
        display_transaction(&item);
       let mut input = String::new();
       if item.amount > 0.0 {
            display_choice(true);
            io::stdin().read_line(&mut input).expect("Failed to read line");
           match input.trim() {
               "1" => income.get_mut("Revenu").unwrap().push(item),
               "2" => income.get_mut("Refund").unwrap().push(item),
               "3" => income.get_mut("Gift").unwrap().push(item),
               "4" => income.get_mut("Other").unwrap().push(item),
               "menu" => {result.push_front(item); break},
               _ => continue,
            }
        } else {
            display_choice(false);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.trim() {
               "1" => outcome.get_mut("Charges").unwrap().push(item),
               "2" => outcome.get_mut("Food").unwrap().push(item),
               "3" => outcome.get_mut("Save").unwrap().push(item),
               "4" => outcome.get_mut("Other").unwrap().push(item),
               "menu" => {result.push_front(item); break},
               _ => continue,
           }
       }
    }
    if result.is_empty() {
        println!("All value have been sorted!");
    } else {
        println!("There are still {} transactions to sort", result.len());
    }
}

fn main() {
    let mut outcome: HashMap<String, Vec<Spent>> = HashMap::new();
    let mut income: HashMap<String, Vec<Spent>> = HashMap::new();

    outcome.insert("Charges".to_string(), Vec::new());
    outcome.insert("Food".to_string(), Vec::new());
    outcome.insert("Save".to_string(), Vec::new());
    outcome.insert("Other".to_string(), Vec::new());
    income.insert("Revenu".to_string(), Vec::new());
    income.insert("Refund".to_string(), Vec::new());
    income.insert("Gift".to_string(), Vec::new());
    income.insert("Other".to_string(), Vec::new());
    let file_content = fs::read_to_string("checkData.json").expect("Unable to read file");
    let mut data: TransactionsData = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    
    let mut result: VecDeque<spent::Spent> = VecDeque::new();
    
    push_transaction_to_result(&mut result, &mut data);
    loop {
        println!("What do you want to do?\n\t 1: Sort \n\t 2: Show categories\n\t 3: quit");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" => sort(&mut result, &mut outcome, &mut income),
            "2" => show_categories(&outcome, &income),
            "3" => break,
            _ => continue,
        }

    }
}

