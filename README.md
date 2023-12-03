# Hot or Not Assignment 
Step-by-step instructions and commands used in setting up and deploying the "Hot or Not Assignment for CRUD operations of a TODO list" project using the Internet Computer (IC) platform.

## Prerequisite installations
Before starting, ensure that you have the DFINITY Canister SDK and the Internet Computer utility (dfx) installed.

## Project Initialization
Rust project for the Hot or Not Todo list crud operations assignment:

## Build and deployment commands
- Update dependencies: 
```
cargo update
```
- Build: 
```
dfx build --network ic hot_or_not_assignment_backend
```
- Install canisters: 
```
dfx canister --network ic install hot_or_not_assignment_backend
```
- Deploy a canister to IC network: 
```
dfx deploy --network ic hot_or_not_assignment_backend
```

## Testing commands
- Unit test: 
```
cargo test
```

## Links to deployment
- Canister link: [https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=tenox-faaaa-aaaag-aceoa-cai](https://a4gq6-oaaaa-aaaab-qaa4q-cai.raw.icp0.io/?id=tenox-faaaa-aaaag-aceoa-cai)

## UI Results
1. UI Result ![UI Result](./src/hot_or_not_assignment_backend/assets/1_Candid_UI.png)
2. Add Task ![Add Task](./src/hot_or_not_assignment_backend/assets/4_Added10_Tasks.png)
3. Get Tasks Using Pagination ![Get Tasks Using Pagination](./src/hot_or_not_assignment_backend/assets/5_Get_Tasks_Pagination.png)
4. Update By Id ![Update By Id](./src/hot_or_not_assignment_backend/assets/8_Update_Task_By_Id.png)
5. Delete By Id ![Delete By Id](./src/hot_or_not_assignment_backend/assets/10_Delete_By_Id.png)
6. Get By Id ![Get By Id](./src/hot_or_not_assignment_backend/assets/11_Get_By_Id.png)


# Test cases Results
Test cases results ![Test cases result](./src/hot_or_not_assignment_backend/assets/12_Test_Results.png)