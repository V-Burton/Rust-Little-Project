use prettytable::{format, Cell, Row, Table};
use std::collections::HashMap;

use crate::Spent;

// fn display_expense(data_bis: &HashMap<String, Vec<Spent>>, target: &str) {
//     let mut table = Table::new();
//     table.add_row(Row::new(vec![
//         Cell::new("reason"),
//         Cell::new("date"),
//         Cell::new("amount"),
//     ]));
//     let mut total: f64 = 0.0;
//     for (category, spends) in data_bis {
//         if *category == target {
//             for spent in spends {
//                 table.add_row(Row::new(vec![Cell::new(&spent.reason), Cell::new(&format!("{}", spent.date)), Cell::new(&format!("{}", spent.amount))]));
//                 total += spent.amount;
//             }
//         }
//     }
//     table.add_row(Row::new(vec![
//         Cell::new("Total").style_spec("b"),
//         Cell::new(&format!("{}", total)).style_spec("b"),
//     ]));

//     table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
//     table.printstd();
// }

pub fn display_transaction(transaction: &Spent){
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new(&transaction.reason), Cell::new(&format!("{}", transaction.date)), Cell::new(&format!("{}", transaction.amount))]));

    table.set_format(*format::consts::FORMAT_NO_TITLE);
    table.printstd();
}

pub fn display_choice(choices: &Vec<&String>){
    let mut table = Table::new();
    let mut row = vec![];
    let mut i = 1;
    row.push(Cell::new("0: menu"));
    for key in choices {
        row.push(Cell::new(&format!("{}: {}", i, key)));
        i += 1;
    }
    row.push(Cell::new(&format!("{}: add categorie", i)));
    table.add_row(Row::new(row));
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
    table.printstd();
}

fn create_table(data: &HashMap<String, Vec<Spent>>, title: &str) -> Table {
    let mut table = Table::new();
    table.set_titles(Row::new(vec![
        Cell::new(title).style_spec("bF"),
        Cell::new("total").style_spec("bF"),
    ]));
    for (category, spends) in data {
        let total: f64 = spends.iter().map(|spent| spent.amount).sum();
        table.add_row(Row::new(vec![Cell::new(&category), Cell::new(&format!("{:.2}", total))]));
    }
    let overall_total: f64 = data.values().flat_map(|spends| spends).map(|spent| spent.amount).sum();
    table.add_row(Row::new(vec![
        Cell::new("Overall Total").style_spec("b"),  // Cellule avec style en gras
        Cell::new(&format!("{:.2}", overall_total)).style_spec("b"),
    ]));
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table
}

fn table_to_string(table: &Table) -> Vec<String> {
    table.to_string().lines().map(|line| line.to_string()).collect()
}

pub fn show_categories(outcome: &HashMap<String, Vec<Spent>>, income: &HashMap<String, Vec<Spent>>) {
    let income_table = create_table(income, "Income");
    let outcome_table = create_table(outcome, "Outcome");

    let income_lines = table_to_string(&income_table);
    let outcome_lines = table_to_string(&outcome_table);

    let max_lines = std::cmp::max(income_lines.len(), outcome_lines.len());
    for i in 0..max_lines {
        let income_line = if i < income_lines.len() { &income_lines[i] } else { "" };
        let outcome_line = if i < outcome_lines.len() { &outcome_lines[i] } else { "" };
        println!("{:<20}    {}", income_line, outcome_line);
    }
}