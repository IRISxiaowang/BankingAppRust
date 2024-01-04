/// This file contains the basic implementation of a banking system.

/// Refer to this for a basic Hasher: https://doc.rust-lang.org/std/hash/trait.Hasher.html
use crate::primitives::*;
use std::{
    collections::hash_map::{DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

pub struct Bank {
    users: HashMap<HashResult, User>,
    balances: HashMap<UserId, Balance>,
    pub(crate) events: Vec<Event>,
    interest_rate: f64,
    tax_rate: f64,
    existential_deposit: Balance,
    user_id_counter: UserId,
}

impl Default for Bank {
    fn default() -> Self {
        Self {
            users: Default::default(),
            balances: Default::default(),
            events: Default::default(),
            interest_rate: INTEREST_RATE,
            tax_rate: TAX_RATE,
            existential_deposit: ED,
            user_id_counter: Default::default(),
        }
    }
}

impl Bank {
    /// Ensure the user is of a given role. If true, return the UserId. Error otherwise.
    fn assert_role(&self, user: HashResult, role: Role) -> BankResult<UserId> {
        match self.users.get(&user) {
            Some(u) => {
                if u.role == role {
                    Ok(u.id)
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }
    }

    /// Log the events to the vec
    fn deposit_event(&mut self, event: Event) {
        self.events.push(event);
    }

    /// Calculate the hash of username and password using a DefaultHasher
    fn hash(username: &String, password: &String) -> HashResult {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        password.hash(&mut hasher);
        hasher.finish()
    }

    /// Function to generate the next user ID (auto-incrementing)
    fn generate_next_user_id(&mut self) -> UserId {
        self.user_id_counter += 1u64;
        self.user_id_counter
    }

    /// Returns true if the given username is already registered.
    /// This function is used to check for duplicated usernames.
    pub fn has_username(&self, username: &String) -> bool {
        self.users
            .iter()
            .any(|(_, user)| user.username == *username)
    }

    /// Add a new user to the `users` hashmap.
    pub fn create_user(
        &mut self,
        username: String,
        password: String,
        role: Role,
    ) -> BankResult<()> {
        if self.has_username(&username) {
            return Err(BankingError::UserAlreadyExist);
        }
        let hash_result = Self::hash(&username, &password);
        let new_user = User {
            id: self.generate_next_user_id(),
            username,
            role,
        };
        self.users.insert(hash_result, new_user);
        Ok(())
    }

    /// Tries to log in with the given username and password. If successful, return the "hash" and role of the
    /// user, which can be used to access other functions.
    pub fn login(&self, username: String, password: String) -> BankResult<(HashResult, Role)> {
        let hash_result = Self::hash(&username, &password);
        match self.users.get(&hash_result) {
            Some(u) => {
                println!("Login ID: {}, Role: {:?}", u.id, u.role);
                Ok((hash_result, u.role))
            }
            None => Err(BankingError::FailedLogin),
        }
    }

    /// Allows the user to set a new password. Rehashes the user and stores the user under the new hash.
    pub fn change_password(&mut self, user: HashResult, new_password: String) -> BankResult<()> {
        let user_data = match self.users.remove(&user) {
            Some(u) => Ok(u),
            None => Err(BankingError::NoUserFound),
        }?;
        let name = user_data.username.clone();
        let new_hash = Self::hash(&name, &new_password);
        self.users.insert(new_hash, user_data);
        Ok(())
    }

    /// Report all the users information and print them into the console.
    /// Requires `manager` or `auditor` role.
    pub fn report(&self, user: HashResult) -> BankResult<()> {
        match self.users.get(&user) {
            Some(u) => {
                if u.role != Role::Customer {
                    self.users.iter().for_each(|(_, user)| {
                        println!("User ID: {}", user.id);
                        println!("Username: {}", user.username);
                        match user.role {
                            Role::Customer => println!("Role: Customer"),
                            Role::Manager => println!("Role: Manager"),
                            Role::Auditor => println!("Role: Auditor"),
                        }
                        if user.role == Role::Customer {
                            let balance = self.balances.get(&user.id).copied().unwrap_or_default();
                            println!("Blance: {}", balance);
                        }
                        println!("------------------------");
                    });
                    Ok(())
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }
    }

    /// Deposits the given `amount` into the user's account.
    /// Requires `Customer` role.
    pub fn deposit(&mut self, user: HashResult, amount: Balance) -> BankResult<()> {
        if amount <= 0f64 {
            return Err(BankingError::InvalidAmount);
        }

        let id = self.assert_role(user, Role::Customer)?;
        let new_balance = match self.balances.get(&id) {
            Some(balance) => Ok(balance + amount),
            None => {
                if amount < self.existential_deposit {
                    Err(BankingError::AmountTooSmall)
                } else {
                    Ok(amount)
                }
            }
        }?;

        self.balances.insert(id, new_balance);
        println!("User: {}, current balance is {}.", id, new_balance);
        // Deposits the balance into the account.
        self.deposit_event(Event::Deposit { id, amount });

        Ok(())
    }

    /// Withdraw `amount` funds from a user's account. If this brings the user's balance
    /// to below ED, the account is reaped.
    /// Requires `Customer` role.
    pub fn withdraw(&mut self, user: HashResult, amount: Balance) -> BankResult<()> {
        if amount <= 0f64 {
            return Err(BankingError::InvalidAmount);
        }

        let id = self.assert_role(user, Role::Customer)?;
        let new_balance = match self.balances.get(&id) {
            Some(balance) => {
                if *balance >= amount {
                    Ok(balance - amount)
                } else {
                    Err(BankingError::InsufficientBalance)
                }
            }
            None => Err(BankingError::InsufficientBalance),
        }?;
        self.deposit_event(Event::Withdrawal { id, amount });
        if new_balance >= self.existential_deposit {
            self.balances.insert(id, new_balance);
            println!("User: {}, current balance is {}.", id, new_balance);
        } else {
            self.balances.remove(&id);
            self.deposit_event(Event::AccountReaped {
                id,
                dust: new_balance,
            });
            println!(
                "User: {}, balance is too low, account is reaped, current balance is 0.",
                id
            );
        }

        Ok(())
    }

    /// Transfer `amount` of fund from the current user to another user.
    /// If the transfer brings the account's balance below ED, the account will be reaped.
    ///
    /// Requires both the current and target user to be `Customer` role.
    pub fn transfer(&mut self, user: HashResult, amount: Balance, target: u64) -> BankResult<()> {
        let id = self.assert_role(user, Role::Customer)?;
        if id == target {
            return Ok(());
        }
        if amount <= 0f64 {
            return Err(BankingError::InvalidAmount);
        }
        if amount < self.existential_deposit {
            return Err(BankingError::AmountTooSmall);
        }

        // Gets the balance of the `to` user
        let mut to_user_balance = match self
            .users
            .iter()
            .find(|(_, user)| user.id == target && user.role == Role::Customer)
        {
            Some(_) => Ok(self.balances.get(&target).copied().unwrap_or_default()),
            None => Err(BankingError::InvalidUserId),
        }?;

        // Calculates the new balance of the current user.
        let new_balance = match self.balances.get(&id) {
            Some(balance) => {
                if *balance >= amount {
                    Ok(balance - amount)
                } else {
                    Err(BankingError::InsufficientBalance)
                }
            }
            None => Err(BankingError::InsufficientBalance),
        }?;

        // Reap the account if below ED, otherwise inser into the hashmap
        if new_balance >= self.existential_deposit {
            self.balances.insert(id, new_balance);
            println!("User: {}, current balance is {}.", id, new_balance);
        } else {
            self.balances.remove(&id);
            self.deposit_event(Event::AccountReaped {
                id,
                dust: new_balance,
            });
            println!(
                "User: {}, balance is too low, account is reaped, current balance is 0.",
                id
            );
        }

        // Inserts the `to` user's balance into the hashmap.
        to_user_balance += amount;
        self.balances.insert(target, to_user_balance);

        self.deposit_event(Event::Transfer {
            id,
            to_id: target,
            amount,
        });
        Ok(())
    }

    /// Returns the current balance of the given user.
    pub fn check_balance(&self, user: HashResult) -> BankResult<Balance> {
        let id = self.assert_role(user, Role::Customer)?;
        Ok(self.balances.get(&id).copied().unwrap_or_default())
    }

    /// Set interest rate, which is used to payout interest to all users.
    /// Requires `Manager` role.
    pub fn set_interest_rate(&mut self, user: HashResult, rate: f64) -> BankResult<()> {
        if rate < 0f64 {
            return Err(BankingError::InvalidInterestRate);
        }
        match self.users.get(&user) {
            Some(u) => {
                if u.role == Role::Manager {
                    self.interest_rate = rate;
                    self.deposit_event(Event::InterestRate {
                        id: u.id,
                        interest_rate: rate,
                    });
                    Ok(())
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }
    }

    /// Sets the tax rate, which is used to take tax from all users.
    /// Requires `Auditor` role.
    pub fn set_tax_rate(&mut self, user: HashResult, rate: f64) -> BankResult<()> {
        if !(0f64..=1f64).contains(&rate) {
            return Err(BankingError::InvalidTaxRate);
        }
        match self.users.get(&user) {
            Some(u) => {
                if u.role == Role::Auditor {
                    self.tax_rate = rate;
                    self.deposit_event(Event::TaxRate {
                        id: u.id,
                        tax_rate: rate,
                    });
                    Ok(())
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }
    }

    /// Pay out interest to all the customers. Increase the balances of all users' by
    /// `interest_rate` proportion.
    /// Requires `Manager` role.
    pub fn pay_interest(&mut self, user: HashResult) -> BankResult<()> {
        match self.users.get(&user) {
            Some(u) => {
                if u.role == Role::Manager {
                    Ok(())
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }?;

        let rate = self.interest_rate;

        // Payout interest to all accounts, and deposit event for each account.
        self.balances
            .iter_mut()
            .map(|(id, balance)| {
                let new_balance = if *balance > Balance::MAX / (1f64 + rate){
                    Balance::MAX
                } else {
                    *balance * (1f64 + rate)
                };
                let interest = new_balance - *balance;
                *balance = new_balance;
                (*id, interest)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(id, interest)| self.deposit_event(Event::Interest { id, interest }));
        Ok(())
    }

    /// Take tax from all the customers. Reduce the balance of all accounts by `tax_rate` proportion.
    /// Requires `Auditor` role.
    pub fn take_tax(&mut self, user: HashResult) -> BankResult<()> {
        match self.users.get(&user) {
            Some(u) => {
                if u.role == Role::Auditor {
                    Ok(())
                } else {
                    Err(BankingError::Unauthorized)
                }
            }
            None => Err(BankingError::NoUserFound),
        }?;
        let rate = self.tax_rate;
        let ed = self.existential_deposit;

        // Reduce balance of all accounts by `tax_rate`. Reap the account if
        // the new balance is below ED.
        self.balances
            .iter_mut()
            .map(|(id, balance)| {
                let tax = *balance * rate;
                *balance *= 1f64 - rate;
                (*id, *balance, tax)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(id, new_balance, tax)| {
                self.deposit_event(Event::Tax { id, tax });
                if new_balance < ed {
                    self.balances.remove(&id);
                    self.deposit_event(Event::AccountReaped {
                        id,
                        dust: new_balance,
                    });
                }
            });
        Ok(())
    }

    // Helper function that iterates all events of a given user and prints them to the console.
    fn iter_event(&self, target_id: UserId) {
        self.events.iter().for_each(|e| {
            if match e {
                Event::Deposit { id: event_id, .. } if *event_id == target_id => true,
                Event::Withdrawal { id: event_id, .. } if *event_id == target_id => true,
                Event::AccountReaped { id: event_id, .. } if *event_id == target_id => true,
                Event::Transfer {
                    id: event_id,
                    to_id,
                    amount: _,
                } if *event_id == target_id || *to_id == target_id => true,
                Event::Interest { id: event_id, .. } if *event_id == target_id => true,
                Event::Tax { id: event_id, .. } if *event_id == target_id => true,
                _ => false,
            } {
                println!("{}", e);
            }
        });
    }

    /// Prints all events related to the current user.
    /// Requires `Customer` role.
    pub fn print_event(&self, user: HashResult) -> BankResult<()> {
        let id = self.assert_role(user, Role::Customer)?;
        println!("===== Events for User ID: {} =====", id);

        // Iterate through the events vector and print matching events
        self.iter_event(id);
        Ok(())
    }

    /// Prints out all events related to the given user
    /// Requires `Manager` or `Auditor` role.
    pub fn print_a_user_event(
        &self,
        user: HashResult,
        role: Role,
        user_id: UserId,
    ) -> BankResult<()> {
        if role == Role::Customer {
            return Err(BankingError::Unauthorized);
        }
        self.assert_role(user, role)?;
        let user_find = self
            .users
            .iter()
            .find(|(_, u)| u.id == user_id && u.role == Role::Customer);
        match user_find {
            Some(_) => Ok(()),
            None => return Err(BankingError::InvalidUserId),
        }?;
        println!("===== Events for User ID: {} =====", user_id);

        // Iterate through the events vector and print matching events
        self.iter_event(user_id);
        Ok(())
    }

    /// Prints all the events logged.
    /// Requires `Manager` or `Auditor role.
    pub fn print_all_events(&self, user: HashResult, role: Role) -> BankResult<()> {
        if role == Role::Customer {
            return Err(BankingError::Unauthorized);
        }
        self.assert_role(user, role)?;
        println!("===== Events for all users =====");

        // Iterate through the events vector and print matching events
        self.events.iter().for_each(|e| println!("{}", e));
        Ok(())
    }
}
