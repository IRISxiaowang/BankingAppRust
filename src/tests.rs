use core::panic;

use crate::{Bank, BankResult, BankingError, Event, HashResult, Role};

#[track_caller]
fn assert_ok<T>(res: BankResult<T>) {
    match res {
        Err(e) => panic!("{}", e),
        _ => (),
    }
}

#[track_caller]
fn assert_noop<T>(res: BankResult<T>, e: BankingError) {
    match res {
        Err(err) => assert!(err == e),
        Ok(_) => panic!("Expected Error: {:?}, but got Ok(_)", e),
    }
}

#[track_caller]
fn assert_last_event(bank: &Bank, e: Event) {
    let last = bank.events[bank.events.len() - 1];
    if last != e {
        panic!("Expected Event: {:?}, but got {}", last, e);
    }
}

fn setup_account(bank: &mut Bank, name: &str, role: Role) -> HashResult {
    assert_ok(bank.create_user(name.to_string(), name.to_string(), role));
    let (hash, _) = bank.login(name.to_string(), name.to_string()).unwrap();

    if role == Role::Customer {
        assert_ok(bank.deposit(hash, 1_000f64));
    }

    hash
}

#[test]
fn can_deposit() {
    // Setup user
    let mut bank = Bank::default();
    let hash = setup_account(&mut bank, "roy", Role::Customer);

    assert_eq!(1_000f64, bank.check_balance(hash).unwrap());
    assert_ok(bank.deposit(hash, 1f64));
    assert_eq!(1_001f64, bank.check_balance(hash).unwrap());
    assert_last_event(
        &bank,
        Event::Deposit {
            id: 1,
            amount: 1f64,
        },
    );

    bank.withdraw(hash, 1_001f64);

    assert_noop(bank.deposit(hash, -100f64), BankingError::InvalidAmount);
    assert_noop(bank.deposit(hash, 2f64), BankingError::AmountTooSmall);
    assert_noop(bank.take_tax(hash), BankingError::Unauthorized);

    assert_eq!(0f64, bank.check_balance(hash).unwrap());
}

#[test]
fn can_withdraw() {
    let mut bank = Bank::default();
    let customer = setup_account(&mut bank, "customer", Role::Customer);
    assert_ok(bank.withdraw(customer, 500f64));
    assert_eq!(500f64, bank.check_balance(customer).unwrap());
}

#[test]
fn can_transfer() {
    let mut bank = Bank::default();
    let hash1 = setup_account(&mut bank, "user1", Role::Customer);
    let hash2 = setup_account(&mut bank, "user2", Role::Customer);

    // Test valid transfer
    assert_ok(bank.transfer(hash1, 500f64, 2));
    assert_eq!(500f64, bank.check_balance(hash1).unwrap());
    assert_eq!(1500f64, bank.check_balance(hash2).unwrap());
    assert_last_event(
        &bank,
        Event::Transfer {
            id: 1,
            to_id: 2,
            amount: 500f64,
        },
    );

    // test error cases
    assert_noop(bank.transfer(hash1, 100f64, 5), BankingError::InvalidUserId);
    assert_noop(
        bank.transfer(hash1, -100f64, 2),
        BankingError::InvalidAmount,
    );
    assert_noop(
        bank.transfer(hash1, 600f64, 2),
        BankingError::InsufficientBalance,
    );

    // transfer to self
    assert_ok(bank.transfer(hash1, 500f64, 1));
    assert_eq!(500f64, bank.check_balance(hash1).unwrap());
    assert_eq!(1500f64, bank.check_balance(hash2).unwrap());

    // test reap account
    assert_ok(bank.transfer(hash1, 496f64, 2));
    assert_eq!(0f64, bank.check_balance(hash1).unwrap());
    assert_eq!(1996f64, bank.check_balance(hash2).unwrap());
    assert_last_event(
        &bank,
        Event::Transfer {
            id: 1,
            to_id: 2,
            amount: 496f64,
        },
    );
}

#[test]
fn can_report() {
    let mut bank = Bank::default();
    // Setup user
    let hash = setup_account(&mut bank, "roy", Role::Customer);
    // Setup user manager
    let hash_manager = setup_account(&mut bank, "manager", Role::Manager);
    // Setup user manager
    let hash_auditor = setup_account(&mut bank, "auditor", Role::Auditor);
    // test pay_interest
    assert_ok(bank.report(hash_manager));
    assert_ok(bank.report(hash_auditor));
    assert_noop(bank.report(hash), BankingError::Unauthorized);
}

#[test]
fn can_pay_interest() {
    let mut bank = Bank::default();
    // Setup user
    let hash = setup_account(&mut bank, "roy", Role::Customer);
    // Setup user manager
    let hash_manager = setup_account(&mut bank, "manager", Role::Manager);
    // test pay_interest
    assert_ok(bank.pay_interest(hash_manager));
    assert_eq!(1010f64, bank.check_balance(hash).unwrap());
}

#[test]
fn can_take_tax() {
    let mut bank = Bank::default();
    // Setup user
    let hash = setup_account(&mut bank, "roy", Role::Customer);
    // Setup user manager
    let hash_auditor = setup_account(&mut bank, "auditor", Role::Auditor);
    // test pay_interest
    assert_ok(bank.take_tax(hash_auditor));
    assert_eq!(980f64, bank.check_balance(hash).unwrap());
}
