use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: String,
    account_id: String,
    amount: Amount,
    descriptions: Descriptions,
    dates: Dates,
    types: Types,
    status: String,
    reference: String,
    providerMutability: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    value: ValueDetail,
    currencyCode: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueDetail {
    unscaledValue: String,
    scale: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Descriptions {
    original: String,
    display: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dates {
    booked: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Types {
    #[serde(rename = "type")]
    kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionsData {
    transactions: Vec<Transaction>,
}

#[derive(Debug, Clone)]
pub struct Spent {
    pub reason: String,
    pub date: String,
    pub amount: f64,
}


pub fn push_transaction_to_result(result: &mut VecDeque<Spent>, data: &mut TransactionsData) {
    data.transactions.reverse();
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
            amount: scaled_value,
        };

        result.push_front(check);
    }
}

// fn calculate_total_expenses(data_bis: &HashMap<String, Vec<Spent>>) -> f64 {
//     data_bis.iter()
//         .filter(|&(category, _)| category != "Save" && category != "Revenu")
//         .flat_map(|(_, spends)| spends)
//         .map(|spent| spent.amount)
//         .sum()
// }