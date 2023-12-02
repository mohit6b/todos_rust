use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

#[derive(Default)]
struct RuntimeState {
    data: Data,
}

#[derive(CandidType, Deserialize, Default)]
struct Data {
    todos: Vec<TodoItem>,
}

#[derive(CandidType, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    name: String,
    done: bool,
}

// Error type for the functions
#[derive(Debug)]
enum TodoError {
    StorageError(ic_cdk::storage::StableStorageError),
}

// Convert storage error to custom TodoError
impl From<ic_cdk::storage::StableStorageError> for TodoError {
    fn from(error: ic_cdk::storage::StableStorageError) -> Self {
        TodoError::StorageError(error)
    }
}

#[pre_upgrade]
fn pre_upgrade() -> Result<(), TodoError> {
    RUNTIME_STATE.with(|state| {
        ic_cdk::storage::stable_save((&state.borrow().data,)).map_err(TodoError::from)
    })?;
    Ok(())
}

#[post_upgrade]
fn post_upgrade() -> Result<(), TodoError> {
    let (data,): (Data,) = ic_cdk::storage::stable_restore().map_err(TodoError::from)?;
    let runtime_state = RuntimeState { data };

    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });

    Ok(())
}

#[update]
fn add(name: String) -> Result<u32, TodoError> {
    RUNTIME_STATE.with(|state| add_impl(name, &mut state.borrow_mut()))
}

fn add_impl(name: String, runtime_state: &mut RuntimeState) -> Result<u32, TodoError> {
    let id = (runtime_state.data.todos.len() as u32) + 1;

    runtime_state.data.todos.push(TodoItem {
        id,
        name,
        done: false,
    });

    Ok(id)
}

#[query]
fn get(offset: u32, limit: u32) -> Result<Vec<TodoItem>, TodoError> {
    RUNTIME_STATE.with(|state| get_impl(&state.borrow(), offset, limit))
}

fn get_impl(runtime_state: &RuntimeState, offset: u32, limit: u32) -> Result<Vec<TodoItem>, TodoError> {
    let total_todos = runtime_state.data.todos.len() as u32;
    
    let start_index = offset.min(total_todos);
    let end_index = (offset + limit).min(total_todos);

    Ok(runtime_state.data.todos[start_index as usize..end_index as usize].to_vec())
}

#[query]
fn get_by_id(id: u32) -> Result<Option<TodoItem>, TodoError> {
    RUNTIME_STATE.with(|state| get_by_id_impl(id, &state.borrow()))
}

fn get_by_id_impl(id: u32, runtime_state: &RuntimeState) -> Result<Option<TodoItem>, TodoError> {
    let item = runtime_state
        .data
        .todos
        .iter()
        .find(|i| i.id == id)
        .cloned();

    Ok(item)
}

#[update]
fn mark_done(id: u32) -> Result<bool, TodoError> {
    RUNTIME_STATE.with(|state| mark_done_impl(id, &mut state.borrow_mut()))
}

fn mark_done_impl(id: u32, runtime_state: &mut RuntimeState) -> Result<bool, TodoError> {
    if let Some(item) = runtime_state.data.todos.iter_mut().find(|i| i.id == id) {
        item.done = true;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[update]
fn delete(id: u32) -> Result<bool, TodoError> {
    RUNTIME_STATE.with(|state| delete_impl(id, &mut state.borrow_mut()))
}

fn delete_impl(id: u32, runtime_state: &mut RuntimeState) -> Result<bool, TodoError> {
    if let Some(index) = runtime_state.data.todos.iter().position(|i| i.id == id) {
        runtime_state.data.todos.remove(index);
        Ok(true)
    } else {
        Ok(false)
    }
}

#[update]
fn update(id: u32, new_name: String) -> Result<bool, TodoError> {
    RUNTIME_STATE.with(|state| update_impl(id, new_name, &mut state.borrow_mut()))
}

fn update_impl(id: u32, new_name: String, runtime_state: &mut RuntimeState) -> Result<bool, TodoError> {
    if let Some(item) = runtime_state.data.todos.iter_mut().find(|i| i.id == id) {
        item.name = new_name;
        Ok(true)
    } else {
        Ok(false)
    }
}
