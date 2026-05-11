use std::collections::HashMap;
use std::io;

// Data structure to represent a bill
struct Bill {
    name: String,
    amount: f64,
}

// Helper to capture trimmed user input from stdin
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    if io::stdin().read_line(&mut buffer).is_ok() {
        let input = buffer.trim().to_string();
        if input.is_empty() {
            None
        } else {
            Some(input)
        }
    } else {
        None
    }
}

// Prompt and parse bill amount from user
fn get_bill_amount() -> Option<f64> {
    println!("Enter bill amount (or type 'back' to cancel):");
    loop {
        let input = get_input()?;
        if input.to_lowercase() == "back" {
            return None;
        }
        match input.parse::<f64>() {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a valid number for the amount (or 'back'):"),
        }
    }
}

fn add_bill(bills: &mut HashMap<String, Bill>) {
    println!("Enter bill name (or type 'back' to cancel):");
    let name = match get_input() {
        Some(input) => {
            if input.to_lowercase() == "back" {
                return;
            }
            input
        }
        None => return,
    };

    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    let bill = Bill {
        name: name.clone(),
        amount,
    };
    bills.insert(name, bill);
    println!("Bill added successfully!");
}

fn view_bills(bills: &HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("--- Current Bills ---");
    for bill in bills.values() {
        println!("Name: {}, Amount: ${:.2}", bill.name, bill.amount);
    }
    println!("---------------------");
}

fn remove_bill(bills: &mut HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("No bills to remove.");
        return;
    }

    view_bills(bills);
    println!("Enter the name of the bill to remove (or type 'back' to cancel):");

    let name = match get_input() {
        Some(input) => {
            if input.to_lowercase() == "back" {
                return;
            }
            input
        }
        None => return,
    };

    if bills.remove(&name).is_some() {
        println!("Bill removed successfully!");
    } else {
        println!("Bill not found.");
    }
}

fn edit_bill(bills: &mut HashMap<String, Bill>) {
    if bills.is_empty() {
        println!("No bills to edit.");
        return;
    }

    view_bills(bills);
    println!("Enter the name of the bill to edit (or type 'back' to cancel):");

    let old_name = match get_input() {
        Some(input) => {
            if input.to_lowercase() == "back" {
                return;
            }
            input
        }
        None => return,
    };

    if !bills.contains_key(&old_name) {
        println!("Bill not found.");
        return;
    }

    println!("Editing bill: {}", old_name);
    println!("Enter new name (leave empty to keep current, or 'back'):");
    let mut new_name = old_name.clone();
    if let Some(input) = get_input() {
        if input.to_lowercase() == "back" {
            return;
        }
        if !input.is_empty() {
            new_name = input;
        }
    }

    println!("Enter new amount (leave empty to keep current, or 'back'):");
    let mut new_amount = bills.get(&old_name).unwrap().amount;
    loop {
        if let Some(input) = get_input() {
            if input.to_lowercase() == "back" {
                return;
            }
            if input.is_empty() {
                break;
            }
            match input.parse::<f64>() {
                Ok(amount) => {
                    new_amount = amount;
                    break;
                }
                Err(_) => println!("Please enter a valid number (or leave empty):"),
            }
        } else {
            break;
        }
    }

    // If the name changed, remove the old entry and insert the new one
    if new_name != old_name {
        bills.remove(&old_name);
    }
    bills.insert(
        new_name.clone(),
        Bill {
            name: new_name,
            amount: new_amount,
        },
    );

    println!("Bill updated successfully!");
}

fn main() {
    let mut bills: HashMap<String, Bill> = HashMap::new();

    loop {
        println!("\n--- Bill Manager Menu ---");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");
        println!("Enter choice (1-5):");

        let choice = get_input();
        match choice.as_deref() {
            Some("1") => add_bill(&mut bills),
            Some("2") => view_bills(&bills),
            Some("3") => remove_bill(&mut bills),
            Some("4") => edit_bill(&mut bills),
            Some("5") => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
