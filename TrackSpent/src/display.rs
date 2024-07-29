use prettytable::{Table, Row, Cell, format};
use std::{collections::HashMap, io::Read};
use std::io;

use crate::Spent;

fn display_expense(data_bis: &HashMap<String, Vec<Spent>>, target: &str) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("reason"),
        Cell::new("date"),
        Cell::new("amount"),
    ]));
    let mut total: f64 = 0.0;
    for (category, spends) in data_bis {
        if category == target {
            for spent in spends {
                table.add_row(Row::new(vec![Cell::new(&spent.reason), Cell::new(&spent.date), Cell::new(&format!("{}", spent.amount))]));
                total += spent.amount;
            }
        }
    }
    table.add_row(Row::new(vec![
        Cell::new("Total").style_spec("b"),
        Cell::new(&format!("{}", total)).style_spec("b"),
    ]));

    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.printstd();
}

pub fn display_transaction(transaction: &Spent){
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new(&transaction.reason), Cell::new(&transaction.date), Cell::new(&format!("{}", transaction.amount))]));

    table.set_format(*format::consts::FORMAT_NO_TITLE);
    table.printstd();
}

pub fn display_choice(income: bool){
    let mut table = Table::new();
    if income {
        table.add_row(Row::new(vec![
            Cell::new("1: Revenu"),
            Cell::new("2: Refund"),
            Cell::new("3: Gift"),
            Cell::new("4: Other"),
            Cell::new("quit"),
        ]));
    } else {
        table.add_row(Row::new(vec![
            Cell::new("1: Charges"),
            Cell::new("2: Food"),
            Cell::new("3: Save"),
            Cell::new("4: Other"),
            Cell::new("quit"),
        ]));
    }
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}

pub fn show_categories(outcome: &HashMap<String, Vec<Spent>>, income: &HashMap<String, Vec<Spent>>) {
    loop {
        println!{"Which category do you want to display?\n\tCharges\n\tFood\n\tSave\n\tOther outcome\n\tRevenu\n\tRefund\n\tGift\n\tOther income\n\tmenu"};
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim() {
            "Charges" => display_expense(&outcome, "Charges"),
            "Food" => display_expense(&outcome, "Food"),
            "Save" => display_expense(&outcome, "Save"),
            "Other outcome" => display_expense(&outcome, "Other"),
            "Revenu" => display_expense(&income, "Revenu"),
            "Refund" => display_expense(&income, "Refund"),
            "Gift" => display_expense(&income, "Gift"),
            "Other income" => display_expense(&income, "Other"),
            "menu" => break,
            _ => continue,
        }
    }

}