/// The main runtime state structure containing data for todo items.
#[derive(Default)]
struct RuntimeState {
    data: Data,
}

/// Data structure for storing a list of todo items.
#[derive(CandidType, Deserialize, Default)]
struct Data {
    todos: Vec<TodoItem>,
}

/// Structure representing a single todo item.
#[derive(CandidType, Deserialize, Clone)]
struct TodoItem {
    id: u32,
    name: String,
    done: bool,
}

/// Function to save the current state before an upgrade.
#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| ic_cdk::storage::stable_save((&state.borrow().data,)).unwrap());
}

/// Function to restore the state after an upgrade.
#[post_upgrade]
fn post_upgrade() {
    let (data,): (Data,) = ic_cdk::storage::stable_restore().unwrap();
    let runtime_state = RuntimeState { 
        data 
    };

    RUNTIME_STATE.with(|state| *state.borrow_mut() = runtime_state);
}

/// Function to add a new todo item.
///
/// # Arguments
///
/// * `name` - The name of the new todo item.
///
/// # Returns
///
/// * The id of the newly added todo item.
#[update]
fn add(name: String) -> u32 {
    RUNTIME_STATE.with(|state| add_impl(name, &mut state.borrow_mut()))
}

/// Implementation of the add operation.
fn add_impl(name: String, runtime_state: &mut RuntimeState) -> u32 {
    let id = (runtime_state.data.todos.len() as u32) + 1;

    runtime_state.data.todos.push(TodoItem {
        id,
        name,
        done: false,
    });

    id
}

/// Function to retrieve a list of todo items with pagination.
///
/// # Arguments
///
/// * `offset` - The starting position for pagination.
/// * `limit` - The maximum number of todos to retrieve.
///
/// # Returns
///
/// * A vector containing references to the todos.
#[query]
fn get(offset: u32, limit: u32) -> Vec<TodoItem> {
    RUNTIME_STATE.with(|state| get_impl(&state.borrow(), offset, limit))
}

/// Implementation of the get operation with pagination.
fn get_impl(runtime_state: &RuntimeState, offset: u32, limit: u32) -> Vec<TodoItem> {
    let total_todos = runtime_state.data.todos.len() as u32;
    
    let start_index = offset.min(total_todos);
    let end_index = (offset + limit).min(total_todos);

    runtime_state.data.todos[start_index as usize..end_index as usize].to_vec()
}

/// Function to retrieve a todo item by its id.
///
/// # Arguments
///
/// * `id` - The id of the todo item to retrieve.
///
/// # Returns
///
/// * An option containing a cloned todo item if found.
#[query]
fn get_by_id(id: u32) -> Option<TodoItem> {
    RUNTIME_STATE.with(|state| get_by_id_impl(id, &state.borrow()))
}

/// Implementation of the get_by_id operation.
fn get_by_id_impl(id: u32, runtime_state: &RuntimeState) -> Option<TodoItem> {
    runtime_state
        .data
        .todos
        .iter()
        .find(|i| i.id == id)
        .cloned()
}

/// Function to mark a todo item as done.
///
/// # Arguments
///
/// * `id` - The id of the todo item to mark as done.
///
/// # Returns
///
/// * `true` if the operation was successful, `false` otherwise.
#[update]
fn mark_done(id: u32) -> bool {
    RUNTIME_STATE.with(|state| mark_done_impl(id, &mut state.borrow_mut()))
}

/// Implementation of the mark_done operation.
fn mark_done_impl(id: u32, runtime_state: &mut RuntimeState) -> bool {
    if let Some(item) = runtime_state.data.todos.iter_mut().find(|i| i.id == id) {
        item.done = true;
        true
    } else {
        false
    }
}

/// Function to delete a todo item by its id.
///
/// # Arguments
///
/// * `id` - The id of the todo item to delete.
///
/// # Returns
///
/// * `true` if the operation was successful, `false` otherwise.
#[update]
fn delete(id: u32) -> bool {
    RUNTIME_STATE.with(|state| delete_impl(id, &mut state.borrow_mut()))
}

/// Implementation of the delete operation.
fn delete_impl(id: u32, runtime_state: &mut RuntimeState) -> bool {
    if let Some(index) = runtime_state.data.todos.iter().position(|i| i.id == id) {
        runtime_state.data.todos.remove(index);
        true
    } else {
        false
    }
}

/// Function to update the name of a todo item by its id.
///
/// # Arguments
///
/// * `id` - The id of the todo item to update.
/// * `new_name` - The new name for the todo item.
///
/// # Returns
///
/// * `true` if the operation was successful, `false` otherwise.
#[update]
fn update(id: u32, new_name: String) -> bool {
    RUNTIME_STATE.with(|state| update_impl(id, new_name, &mut state.borrow_mut()))
}

/// Implementation of the update operation.
fn update_impl(id: u32, new_name: String, runtime_state: &mut RuntimeState) -> bool {
    if let Some(item) = runtime_state.data.todos.iter_mut().find(|i| i.id == id) {
        item.name = new_name;
        true
    } else {
        false
    }
}
