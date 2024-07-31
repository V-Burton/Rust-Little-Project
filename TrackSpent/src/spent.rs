use std::collections::VecDeque;
use chrono::NaiveDate;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    amount: Amount,
    descriptions: Descriptions,
    dates: Dates,
    types: Types,
    status: String,
    reference: String,
    #[serde(rename = "providerMutability")]
    provider_mutability: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Amount {
    value: ValueDetail,
    #[serde(rename = "currencyCode")]
    currency_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ValueDetail {
    #[serde(rename = "unscaledValue")]
    unscaled_value: String,
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
    pub date: NaiveDate,
    pub amount: f64,
}


pub fn push_transaction_to_result(result: &mut VecDeque<Spent>, data: &mut TransactionsData) {
    data.transactions.reverse();
    for transaction in &data.transactions{
        let display = &transaction.descriptions.display;
        let date = &transaction.dates.value;
        let value = &transaction.amount.value.unscaled_value;
        let scale = transaction.amount.value.scale.parse::<i32>().expect("Unable to parse scale");
        let unscaled_value = value.parse::<f64>().expect("Unable to parse unscaled value");
        let scaled_value = unscaled_value / 10f64.powi(scale);
        
        let check = Spent {
            reason: display.clone(),
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("Unable to parse date"),
            amount: scaled_value,
        };

        result.push_front(check);
    }
}