#![allow(dead_code)]

use std::fmt::Display;

pub const INTEREST_RATE: f64 = 0.01f64;
pub const TAX_RATE: f64 = 0.02f64;
pub const ED: f64 = 5f64;
pub type UserId = u64;
pub type Balance = f64;

// Default hash output of `DefaultHasher`
pub type HashResult = u64;

pub type BankResult<T> = Result<T, BankingError>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Role {
    Customer,
    Manager,
    Auditor,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub role: Role,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BankingError {
    Unauthorized,
    InsufficientBalance,
    InvalidAmount,
    FailedLogin,
    NoUserFound,
    AmountTooSmall,
    InvalidUserId,
    InvalidTaxRate,
    InvalidInterestRate,
    UserAlreadyExist,
}

/// Display user facing message for each error
impl std::fmt::Display for BankingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankingError::Unauthorized => {
                write!(f, "Current user is not authorized to do this operation.")
            }
            BankingError::InsufficientBalance => write!(f, "User does not have enough balance."),
            BankingError::InvalidAmount => write!(f, "The amount given is not valid."),
            BankingError::FailedLogin => {
                write!(f, "Login failed! The username or password is not correct.")
            }
            BankingError::NoUserFound => write!(f, "Error, User does not exist."),
            BankingError::AmountTooSmall => write!(f, "Error, the amount given is too small."),
            BankingError::InvalidUserId => write!(f, "Error, user ID is not exist."),
            BankingError::InvalidTaxRate => write!(f, "Error, tax rate must be between 0 and 1."),
            BankingError::InvalidInterestRate => {
                write!(f, "Error, interest rate could not be nagitive.")
            }
            BankingError::UserAlreadyExist => write!(f, "Error, this user is already exist."),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Event {
    Deposit {
        id: UserId,
        amount: Balance,
    },
    Withdrawal {
        id: UserId,
        amount: Balance,
    },
    AccountReaped {
        id: UserId,
        dust: Balance,
    },
    Transfer {
        id: UserId,
        to_id: UserId,
        amount: Balance,
    },
    Interest {
        id: UserId,
        interest: Balance,
    },
    Tax {
        id: UserId,
        tax: Balance,
    },
    InterestRate {
        id: UserId,
        interest_rate: f64,
    },
    TaxRate {
        id: UserId,
        tax_rate: f64,
    },
}
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Deposit { id, amount } => {
                write!(f, "User ID: {}, Deposit - Amount: {}", id, amount)
            }
            Event::Withdrawal { id, amount } => {
                write!(f, "User ID: {}, Withdrawal - Amount: -{}", id, amount)
            }
            Event::AccountReaped { id, dust } => {
                write!(f, "User ID: {}, Account Reaped - Dust: {}", id, dust)
            }
            Event::Transfer { id, to_id, amount } => write!(
                f,
                "Transfer - Amount: {}, From ID: {}, To ID: {}",
                amount, id, to_id
            ),
            Event::Interest { id, interest } => {
                write!(f, "User ID: {}, Interest - Amount: {}", id, interest)
            }
            Event::Tax { id, tax } => write!(f, "User ID: {}, Tax - Amount: -{}", id, tax),
            Event::InterestRate { id, interest_rate } => {
                write!(f, "User ID: {}, Interest Rate - Set: {}", id, interest_rate)
            }
            Event::TaxRate { id, tax_rate } => {
                write!(f, "User ID: {}, Tax Rate - Set: {}", id, tax_rate)
            }
        }
    }
}
