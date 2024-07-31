use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;

use crate::display;
use crate::spent;

use spent::Spent;
use display::{display_choice, display_transaction};

fn add_key(map: &mut HashMap<String, Vec<Spent>>) {
    let mut input = String::new();
    println!{"Please enter the name of the new category: "};
    io::stdin().read_line(&mut input).expect("Failed to read line");
    map.insert(input.trim().to_string(), Vec::new());
}

pub fn sort(result: &mut VecDeque<spent::Spent>, outcome: &mut HashMap<String, Vec<Spent>>, income: &mut HashMap<String, Vec<Spent>>) {
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