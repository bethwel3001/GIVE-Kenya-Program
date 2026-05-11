use std::io;

// Data structure to represent a bill
struct Bill {
    name: String,
    amount: f64,
}

// Helper to capture trimmed user input from stdin
fn get_input() -> Option<String> {
    let mut buffer = String::new();
    // Read input from the terminal
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
    println!("Enter bill amount:");
    loop {
        let input = get_input()?;
        // Parse input to f64
        match input.parse::<f64>() {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please enter a valid number for the amount:"),
        }
    }
}

fn add_bill(bills: &mut Vec<Bill>) {
    println!("Enter bill name:");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };

    let amount = match get_bill_amount() {
        Some(amount) => amount,
        None => return,
    };

    // Add the new bill to the collection
    let bill = Bill { name, amount };
    bills.push(bill);
    println!("Bill added successfully!");
}

fn view_bills(bills: &Vec<Bill>) {
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("--- Current Bills ---");
    for bill in bills {
        println!("Name: {}, Amount: ${:.2}", bill.name, bill.amount);
    }
    println!("---------------------");
}

fn main() {
    // In-memory storage for bills
    let mut bills: Vec<Bill> = Vec::new();

    loop {
        println!("\n--- Bill Manager Menu ---");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Exit");
        println!("Enter choice (1-3):");

        let choice = get_input();
        match choice.as_deref() {
            Some("1") => add_bill(&mut bills),
            Some("2") => view_bills(&bills),
            Some("3") => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
