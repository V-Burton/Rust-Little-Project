use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{File, OpenOption};
use serde::Serialize
use serde::Deserialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Budget {
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    pub Name: String,
    pub expenses: Vec<Spent>,
    pub total: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Spent {
    pub reason: String,
    pub amount: usize, 
}

impl Spent {
    pub fn new(text: String, amount: usize) -> Spent {
        Spent {text, amount}
    }
}

impl Category {
    pub fn new(title: String) -> Category {
        Category {title, Vec<Spent>::new, 0}
    }
}

impl Budget {
    pub fn new(categories: Vec<Category>) -> budget{
        Budget {categories}
    }
}

impl fmt::Display for Budget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = total(&self);
        for category in self.categories {
            let sub_total = total_by_category(&self, category);
            write!(f, "- {}: {} ({})\n", category, sub_total, (sub_total / total) * 100)
        }
        write!(f, "Total spent: {}.\n", total);
    }

}

fn total(&self: Budget) -> usize {
    let total = 0;
    for category in self.categories {
        total += category.total;
    }
    total
}

fn total_by_category(&category: Category>) -> usize {
    let total = 0;
    for spent in category.expenses {
        total += spent.amount;
    }
    category.total = total;
    total
}

fn add_spent()