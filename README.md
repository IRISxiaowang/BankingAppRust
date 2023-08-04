# Overview
This excercise provides a chance to experience basic accounting/banking app systems as well as the use of Iterator functions.

This app contains a Banking application made from 3 parts:
1. CLI front end that provides users with basic functions such as deposit, and withdraw etc.
2. A User permission management system that allows users and their Roles to be managed.
3. Hashmap mock database backend that provides storage. No SQL is involved, but Iterators
   should be used as much as possible.

## Product requirement

### 1. Basic CLI front end.

a. User can interact with the app via a CLI. Create a simple menu first, then add commands and functions as you go.

### 2. User Registration

a. Users must register with a unique username and password. The Hash of these are stored in a Hashmap

b. Users also have different Roles, which gives them permission to do certain operations.

### 3. User Login and management

a. User can login with their username and password

b. Logged in user is stored as a "Hash" in the front end, which is passed into functions to check for permissions.

c. User can change their password, but not their Role and username.

### 4. Basic accounting system.

a. Role::Customer can deposit funds and withdraw funds

b. Role::Customer can transfer funds from their account to another user's account by ID

c. Role::Manager can to pay out interest, calculated using a defined interest rate (Increase balance of all accounts by interest_rate%)

d. Role::Auditor can demand a Tax payment from all users. Tax rate is provided by the Auditor (Reduce balance of all accounts by a %).

e. All account must contain a minimum balance ($5) - this is called ExistentialDeposit.
    i. Deposits that results in balances less than the ED are rejected.
    ii. After Withdraws, the amount less than ED are "Reaped" - Balance entry is removed from the Hashmap. Account MUST REMAIN.
    iii. Events are logged when accounts are reaped.

f. Role::Manager can update the current interest rate and existential deposit.
   
### 5. Auditing and Book-keeping

a. Manager and Auditor can request a full report of all users in the format of:
    UserId, UserName, Role, Balance

b. All balance changes are tracked by "Events", and stored in a Log Book

c. All events in the log book are queriable:
    i. Get all events that involed a specific user by ID
    i. Get all Deposit+Withdrawal events.

### 6. Other requirements
a. All banking operations need to verify Permission (via Hash passed in) and only execute if the user has the correct Role.

b. Returns Custom Enum error that can be propogated up to the CLI.

c. All error handling should be done in a single function

d. Unit tests are written for some function.

e. Non-trivial functions should have documentation.

f. Write a test shell scripts that tests some functions.
