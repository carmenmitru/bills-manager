use std::collections::HashMap;
use std::io;

/// "Interface" for Bill
#[derive(Debug, Clone)]
pub struct Bill {
    name: String,
    amount: f64,
}

/// Collection used to store bills.
pub struct Bills {
    inner: HashMap<String, Bill>,
}

impl Bills {
    /// Create a new bills collection.
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    /// Add a new bill. If a bill with the same name exists, it is overwritten.
    fn add(&mut self, bill: Bill) {
        self.inner.insert(bill.name.clone(), bill);
    }

    /// Retrieve bills array
    fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    /// Removes an existing bill. Returns false if the bill does not exist.
    fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    /// Updates an existing bill. Returns false if the bill does not exist.
    fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            }
            None => false,
        }
    }
}

/// Retrieves user input. 
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    } else {
        Some(input)
    }
}

/// Retrieves a bill amount. 
fn get_bill_amount() -> Option<f64> {
    println!("Amount:");
    loop {
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a number"),
        }
    }
}

mod menu {
    use crate::{get_bill_amount, get_input, Bill, Bills};

    /// Add bill
    pub fn add_bill(bills: &mut Bills) {
        println!("PLease insert bill name:");
        let name = match get_input() {
            Some(input) => input,
            None => return,
        };
        let amount = match get_bill_amount() {
            Some(amount) => amount,
            None => return,
        };
        let bill = Bill { name, amount };
        bills.add(bill);
        println!("Bill added");
    }

    // Remove a bill
    pub fn remove_bill(bills: &mut Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
        println!("Enter bill name to remove:");
        let name = match get_input() {
            Some(name) => name,
            None => return,
        };
        if bills.remove(&name) {
            println!("removed");
        } else {
            println!("bill not found");
        }
    }

    /// Process for viewing existing bills.
    pub fn view_bills(bills: &Bills) {
        for bill in bills.get_all() {
            println!("{:?}", bill);
        }
    }
}

// Menu options
enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
}

impl MainMenu {
    fn from_str(input: &str) -> Option<MainMenu> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBill),
            "3" => Some(Self::RemoveBill),
            _ => None,
        }
    }

    // Show menu options
    fn show() {
        println!("");
        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("");
        println!("Enter selection:");
    }
}

/// Main menu 
/// User can select an option
fn run_main_menu() -> Option<String> {
    let mut bills = Bills::new();

    loop {
        MainMenu::show();
        let input = get_input()?;
        match MainMenu::from_str(input.as_str()) {
            Some(MainMenu::AddBill) => menu::add_bill(&mut bills),
            Some(MainMenu::ViewBill) => menu::view_bills(&bills),
            Some(MainMenu::RemoveBill) => menu::remove_bill(&mut bills),
            None => break,
        }
    }
    None
}

fn main() {
    run_main_menu();
}