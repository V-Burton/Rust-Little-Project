use std::io;
use std::collections::HashMap;
use std::fs;
use std::collections::VecDeque;

mod display;
mod spent;
mod sort;

use spent::{push_transaction_to_result, Spent, TransactionsData};
use display::show_categories;
use sort::sort;


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

