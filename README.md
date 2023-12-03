# Hot or Not Assignment 
This readme provides step-by-step instructions and commands used in setting up and deploying the "Hot or Not Assignment for CRUD operations of a TODO list" project using the Internet Computer (IC) platform.

# Prerequisites
Before starting, ensure that you have the DFINITY Canister SDK and the Internet Computer utility (dfx) installed. You can find installation instructions here.

# Project Initialization
Create a new Rust project for the Hot or Not Assignment:


# Project Commands and Explanations

## 1. dfx --version
Displays the version of the dfx command-line tool.

```dfx identity whoami
Displays information about the current authenticated identity in the dfx project.

```dfx new --type=rust hot_or_not_assignment
Creates a new Rust project named "hot_or_not_assignment" using dfx.

## 4. cd hot_or_not_assignment
Changes the current directory to "hot_or_not_assignment".

## 5. dfx ping ic
Pings the Internet Computer (IC) network to check connectivity.

## 6. dfx identity whoami
Displays information about the current authenticated identity.

## 7. dfx identity get-principal
Retrieves the principal ID associated with the authenticated identity.

## 8. dfx ledger account-id
Displays the account ID associated with the current identity.

## 9. dfx ledger --network ic balance
Checks the balance of the ledger associated with the IC network.

## 10. dfx identity --network ic get-wallet
Retrieves the wallet associated with the IC network for the authenticated identity.

## 11. dfx ledger --network ic balance
Checks the balance of the ledger associated with the IC network.

## 12. dfx identity new hotornot
Creates a new identity named "hotornot" in the dfx project.

## 13. dfx identity use hotornot
Switches the active identity to "hotornot".

## 14. dfx identity whoami
Displays information about the currently active identity.

## 15. dfx identity list
Lists all available identities in the dfx project.

## 16. dfx identity get-principal
Retrieves the principal ID associated with the authenticated identity.

## 17. dfx --version
Displays the version of the dfx command-line tool.

## 18. dfx wallet --network=ic balance
Checks the balance of the wallet canister associated with the IC network.

## 19. dfx ledger --network ic balance
Checks the balance of the ledger associated with the IC network.

## 20. dfx ledger account-id
Displays the account ID associated with the current identity.

## 21. dfx ledger --network ic create-canister xyxgy-arfar-srav3-oub7e-xe5ho-5m5s3-o4c4z-wfv3v-wq6e6-onshm-zae --amount 0.7
Creates a canister with a specified ID and initializes it with cycles.

## 22. dfx identity --network ic deploy-wallet trk72-eiaaa-aaaag-acenq-cai
Deploys a wallet canister to the IC network with the specified identifier.

## 23. dfx wallet --network ic balance
Checks the balance of the wallet canister associated with the IC network.

## 24. dfx canister --network ic call "trk72-eiaaa-aaaag-acenq-cai" authorize '(principal "xyxgy-arfar-srav3-oub7e-xe5ho-5m5s3-o4c4z-wfv3v-wq6e6-onshm-zae")'
Authorizes a principal for the specified wallet canister.

## 25. npm install
Installs the Node.js package dependencies for the project.

## 26. dfx canister --network ic create hot_or_not_assignment_backend --with-cycles 100000000000
Creates a canister named "hot_or_not_assignment_backend" on the IC network with an initial cycle endowment.

## 27. dfx wallet --network ic balance
Checks the balance of the wallet canister associated with the IC network.

## 28. dfx build --network ic hot_or_not_assignment_backend
Builds the canister code for deployment on the IC network.

## 29. dfx deploy --network ic hot_or_not_assignment_backend
Deploys the canister code to the IC network.

## 30. dfx canister deposit-cycles 1000000000000 tenox-faaaa-aaaag-aceoa-cai --network ic
Deposits cycles into a specified canister using its identifier.

## 31. dfx wallet --network ic balance
Checks the balance of the wallet canister associated with the IC network.

## 32. dfx canister --network ic call hot_or_not_assignment_backend add_todo '("Complete Assignment": text)'
Calls the "add_todo" method on the canister to add a todo item.

## 33. dfx canister --network ic install hot_or_not_assignment_backend
Installs the code of the canister back onto the IC network.

## 34. dfx canister --network ic call hot_or_not_assignment_backend get_todo '("1": usize)'
Calls the "get_todo" method on the canister to retrieve a todo item by ID.

## 35. dfx canister --network ic call hot_or_not_assignment_backend add_todo '("Complete Assignment": text)'
Calls the "add_todo" method on the canister to add a todo item.

## 36. dfx build --network ic hot_or_not_assignment_backend
Builds the canister code for deployment on the IC network.

## 37. dfx canister --network ic install hot_or_not_assignment_backend
Installs the code of the canister back onto the IC network.

## 38. dfx deploy --network ic hot_or_not_assignment_backend
Deploys the canister code to the IC network.

## 39. dfx canister --network ic --uninstall-code hot_or_not_assignment_backend
Uninstalls the code of the canister, effectively removing it from the IC network.

## 40. cargo update
Updates Rust dependencies using Cargo.

## 41. cargo test
Runs tests for the Rust project.
