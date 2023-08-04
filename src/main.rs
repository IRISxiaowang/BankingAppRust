#![allow(unused_must_use)]

mod bank;
mod primitives;

#[cfg(test)]
mod tests;

pub use bank::Bank;
pub use primitives::*;
use std::io;

// Helper function: Prints the error message on failure.
fn parse_result(res: BankResult<()>) {
    match res {
        Ok(()) => println!("Completed."),
        Err(e) => println!("Error: {}", e),
    }
}

/// Page used to register a new user.
fn register_page(bank: &mut Bank) {
    let mut username = String::new();
    let mut password = String::new();
    let mut role = String::new();
    println!("===== Register page =====");

    // Ask from the user a unique username
    while username.is_empty() || bank.has_username(&username) {
        println!("Please input your user name(unique):");
        username.clear();
        io::stdin().read_line(&mut username);
        username = username.trim().to_string();
    }

    // Ask from the user a Password
    while password.is_empty() {
        println!("Please input your password(not null):");
        password.clear();
        io::stdin().read_line(&mut password);
        password = password.trim().to_string();
    }

    // Ask from the user a Role
    println!("Please choose your role: 1.Customer; 2.Manager; 3.Auditor;");
    io::stdin().read_line(&mut role);
    match role.trim() {
        "1" => parse_result(bank.create_user(username, password, Role::Customer)),
        "2" => parse_result(bank.create_user(username, password, Role::Manager)),
        "3" => parse_result(bank.create_user(username, password, Role::Auditor)),
        _ => {
            println!("Exiting...");
        }
    }
}

/// Page used to log in.
fn login_page(bank: &Bank) -> BankResult<(HashResult, Role)> {
    let mut username = String::new();
    let mut password = String::new();
    println!("=====  Login page  =====");

    println!("Please input your username:");
    io::stdin().read_line(&mut username);
    username = username.trim().to_string();

    println!("Please input your password:");
    io::stdin().read_line(&mut password);
    password = password.trim().to_string();

    bank.login(username, password)
}

/// Page used for users of `Customer` role
fn customer_page(bank: &mut Bank, user: HashResult) {
    let mut user_input = String::new();
    println!("=====  Customer page  =====");
    loop {
        println!("Please choose: 1.Deposit; 2.Withdraw; 3.Transfer; 4.Change Password; 5.Print Events; 6.Check Balance; 7.Quit;");
        user_input.clear();
        io::stdin().read_line(&mut user_input);
        match user_input.trim() {
            "1" => {
                println!("Please input how much money you want to deposit:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount);
                // Delete the \n from the input
                amount.pop();
                match amount.parse::<f64>() {
                    Ok(converted_amount) => parse_result(bank.deposit(user, converted_amount)),
                    Err(e) => {
                        println!("Please input a number! {}", e);
                    }
                };
            }
            "2" => {
                println!("Please input how much money you want to withdraw:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount);
                // Delete the \n from the input
                amount.pop();
                match amount.parse() {
                    Ok(converted_amount) => parse_result(bank.withdraw(user, converted_amount)),
                    Err(_) => {
                        println!("Please input a number!");
                    }
                };
            }
            "3" => {
                println!("Please input how much money you want to transfer:");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount);
                // Delete the \n from the input
                amount.pop();
                let converted_amount: f64 = match amount.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please input a number!");
                        continue;
                    }
                };
                println!("Please input the ID you want to transfer to:");
                let mut to_id = String::new();
                io::stdin().read_line(&mut to_id);
                // Delete the \n from the input
                to_id.pop();
                let converted_id: u64 = match to_id.parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please input a number!");
                        continue;
                    }
                };
                parse_result(bank.transfer(user, converted_amount, converted_id));
            }
            "4" => {
                println!("Please inter your new password:");
                let mut change_password = String::new();
                io::stdin().read_line(&mut change_password);
                change_password = change_password.trim().to_string();
                parse_result(bank.change_password(user, change_password));
                return;
            }
            "5" => parse_result(bank.print_event(user)),
            "6" => {
                println!("Current balance is {}", bank.check_balance(user).unwrap());
            }
            "7" => {
                println!("Quit...");
                return;
            }
            _ => println!("Invalid input. Please try again."),
        }
    }
}

/// Page used for users of `Manager` role
fn manager_page(bank: &mut Bank, user: HashResult) {
    let mut user_input = String::new();
    println!("=====  Manager page  =====");
    loop {
        println!("Please choose: 1.Report; 2.Set interest rate; 3.Pay interest; 4.Change Password; 5.Print a user's events; 6.Print all events; 7.Quit;");
        user_input.clear();
        io::stdin().read_line(&mut user_input);
        match user_input.trim() {
            "1" => parse_result(bank.report(user)),
            "2" => {
                println!("Please inter the interest rate:");
                let mut interest_rate = String::new();
                io::stdin().read_line(&mut interest_rate);
                // Delete the \n from the input
                interest_rate.pop();
                match interest_rate.parse() {
                    Ok(num) => parse_result(bank.set_interest_rate(user, num)),
                    Err(_) => {
                        println!("Please input a number!");
                    }
                };
            }
            "3" => parse_result(bank.pay_interest(user)),
            "4" => {
                println!("Please inter your new password:");
                let mut change_password = String::new();
                io::stdin().read_line(&mut change_password);
                parse_result(bank.change_password(user, change_password));
                return;
            }
            "5" => {
                println!("Please input a user id:");
                let mut user_id = String::new();
                io::stdin().read_line(&mut user_id);
                // Delete the \n from the input
                user_id.pop();
                match user_id.parse() {
                    Ok(num) => parse_result(bank.print_a_user_event(user, Role::Manager, num)),
                    Err(_) => {
                        println!("Please input a number!");
                    }
                };
            }
            "6" => parse_result(bank.print_all_events(user, Role::Manager)),
            "7" => {
                println!("Quit...");
                return;
            }
            _ => println!("Invalid input. Please try again."),
        };
    }
}

/// Page used for users of `Auditor` role
fn auditor_page(bank: &mut Bank, user: HashResult) {
    let mut user_input = String::new();
    println!("=====  Auditor page  =====");
    loop {
        println!("Please choose: 1.Report; 2.Set tax rate; 3.Take tax; 4.Change Password; 5.Print a user's events; 6.Print all events; 7.Quit;");
        user_input.clear();
        io::stdin().read_line(&mut user_input);
        match user_input.trim() {
            "1" => parse_result(bank.report(user)),
            "2" => {
                println!("Please inter the tax rate:");
                let mut tax_rate = String::new();
                io::stdin().read_line(&mut tax_rate);
                // Delete the \n from the input
                tax_rate.pop();
                match tax_rate.parse() {
                    Ok(num) => parse_result(bank.set_tax_rate(user, num)),
                    Err(_) => {
                        println!("Please input a number!");
                    }
                };
            }
            "3" => parse_result(bank.take_tax(user)),
            "4" => {
                println!("Please inter your new password:");
                let mut change_password = String::new();
                io::stdin().read_line(&mut change_password);
                bank.change_password(user, change_password);
                return;
            }
            "5" => {
                println!("Please input a user id:");
                let mut user_id = String::new();
                io::stdin().read_line(&mut user_id);
                // Delete the \n from the input
                user_id.pop();
                match user_id.parse() {
                    Ok(num) => parse_result(bank.print_a_user_event(user, Role::Auditor, num)),
                    Err(_) => {
                        println!("Please input a number!");
                    }
                };
            }
            "6" => parse_result(bank.print_all_events(user, Role::Auditor)),
            "7" => {
                println!("Quit...");
                return;
            }
            _ => println!("Invalid input. Please try again."),
        }
    }
}

/// Main CLI page.
fn cli() {
    let mut bank = Bank::default();
    let mut user_input = String::new();
    loop {
        println!("Welcome to ANZ bank!");
        println!("Please choose: 1. Login; 2.Register; 3.Exit;");
        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input.");
        match user_input.trim() {
            "1" => match login_page(&bank) {
                Ok((user, role)) => match role {
                    Role::Customer => customer_page(&mut bank, user), //different menu pass in hash
                    Role::Manager => manager_page(&mut bank, user),
                    Role::Auditor => auditor_page(&mut bank, user),
                },
                Err(e) => println!("{e}"),
            },
            "2" => register_page(&mut bank),
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid input. Try again."),
        }
    }
}

fn main() {
    cli();
}
