Hot or Not Assignment - Readme
This readme provides step-by-step instructions and commands used in setting up and deploying the "Hot or Not Assignment" project using the Internet Computer (IC) platform.

Prerequisites
Before starting, ensure that you have the DFINITY Canister SDK and the Internet Computer utility (dfx) installed. You can find installation instructions here.

Project Initialization
Create a new Rust project for the Hot or Not Assignment:



dfx new --type=rust hot_or_not_assignment
cd hot_or_not_assignment
Initialize the identity:



dfx identity new hotornot
dfx identity use hotornot
Build and Deploy Backend
Build and deploy the backend canister:



dfx canister --network ic create hot_or_not_assignment_backend --with-cycles 100_000_000_000
dfx build --network ic hot_or_not_assignment_backend
dfx deploy --network ic hot_or_not_assignment_backend
Interact with the backend:



dfx canister --network ic call hot_or_not_assignment_backend add_todo '("Complete Assignment": text)'
Additional Commands
Check wallet balance:



dfx wallet --network ic balance
Deposit cycles into an account:



dfx canister deposit-cycles 1000000000000 tenox-faaaa-aaaag-aceoa-cai --network ic
Install and call functions:



dfx canister --network ic install hot_or_not_assignment_backend
dfx canister --network ic call hot_or_not_assignment_backend hello '(" word")'
Additional Notes
The project directory structure and file creations:



ls
Check DFINITY version:



dfx --version