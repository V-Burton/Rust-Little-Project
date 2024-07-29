use std::io;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    id: String,
    accountId: String,
    amount: Amount,
    descriptions: Descriptions,
    dates: Dates,
    types: Types,
    status: String,
    reference: String,
    providerMutability: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct Amount {
    value: ValueDetail,
    currencyCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ValueDetail {
    unscaledValue: String,
    scale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Descriptions {
    original: String,
    display: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dates {
    booked: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Types {
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TransactionsData {
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
struct Spent {
    reason: String,
    date: String,
    amount: f64,
}

fn push_transaction_to_result(result: &mut Vec<Spent>, data: &TransactionsData) {
    for transaction in &data.transactions{
        let display = transaction.descriptions.display.clone();
        let date = transaction.dates.value.clone();
        let value = transaction.amount.value.unscaledValue.clone();
        let scale = transaction.amount.value.scale.parse::<i32>().expect("Unable to parse scale");
        let unscaled_value = value.parse::<f64>().expect("Unable to parse unscaled value");
        let scaled_value = unscaled_value / 10f64.powi(scale);
        
        let check = Spent {
            reason: display.clone(),
            date: date.clone(),
            amount: scaled_value as f64,
        };

        result.push(check);
    }
}

fn calculate_total_expenses(dataBis: &HashMap<String, Vec<Spent>>) -> f64 {
    dataBis.iter()
        .filter(|&(category, _)| category != "Save" && category != "Revenu")
        .flat_map(|(_, spends)| spends)
        .map(|spent| spent.amount)
        .sum()
}

fn main() {
    let file_content = fs::read_to_string("checkData.json").expect("Unable to read file");
    let data: TransactionsData = serde_json::from_str(&file_content).expect("Unable to parse JSON");
    let mut dataBis: HashMap<String, Vec<Spent>> = HashMap::new();

    dataBis.insert("Charges".to_string(), Vec::new());
    dataBis.insert("Food".to_string(), Vec::new());
    dataBis.insert("Save".to_string(), Vec::new());
    dataBis.insert("Other".to_string(), Vec::new());
    dataBis.insert("Revenu".to_string(), Vec::new());
    
    let mut result: Vec<Spent> = Vec::new();
    
    push_transaction_to_result(&mut result, &data);
    
    for item in &result {
        println!("{} | {} | {}\n", item.reason, item.date, item.amount);
        
        let mut input = String::new();
        println!("\tEnter the category of the transaction: \n\t 1: Charges \n\t 2: Food \n\t 3: Save \n\t 4: Other \n\t 5: Revenu \n\tIf you want to consult the data of one of the categories type the name of the category.");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "1" => dataBis.get_mut("Charges").unwrap().push(item.clone()),
            "2" => dataBis.get_mut("Food").unwrap().push(item.clone()),
            "3" => dataBis.get_mut("Save").unwrap().push(item.clone()),
            "4" => dataBis.get_mut("Other").unwrap().push(item.clone()),
            "5" => dataBis.get_mut("Revenu").unwrap().push(item.clone()),
            "Charges" => {
                let totalCharges: f64 = dataBis.get("Charges").unwrap().iter().map(|x| x.amount).sum();
                let totalAll: f64 = calculate_total_expenses(&dataBis);
                for item in dataBis.get("Charges").unwrap() {
                    println!("{} | {} | {}", item.reason, item.date, item.amount);
                }
                println!("Total Charges: {} of total: {}", totalCharges, totalAll);
            },
            "Food" => {
                for item in dataBis.get("Food").unwrap() {
                    println!("{} | {} | {}", item.reason, item.date, item.amount);
                }
            },
            "Save" => {
                for item in dataBis.get("Save").unwrap() {
                    println!("{} | {} | {}", item.reason, item.date, item.amount);
                }
            },
            "Other" => {
                for item in dataBis.get("Other").unwrap() {
                    println!("{} | {} | {}", item.reason, item.date, item.amount);
                }
            },
            "Revenu" => {
                for item in dataBis.get("Revenu").unwrap() {
                    println!("{} | {} | {}", item.reason, item.date, item.amount);
                }
            },
            "quit" => break,
            _ => continue,
        }
    }
}

