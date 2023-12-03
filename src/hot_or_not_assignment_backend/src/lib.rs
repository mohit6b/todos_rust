use candid::CandidType;
use ic_cdk_macros::*;
use serde::Deserialize;
use std::cell::RefCell;

thread_local! {
    // If RuntimeState doesn't implement Default you can wrap it in an Option instead
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

/// The main runtime state structure containing data for todo items.
#[derive(Default)]
struct RuntimeState {
    data: Data
}

/// Data structure for storing a list of todo items.
#[derive(CandidType, Deserialize, Default)]
struct Data {
    todos: Vec<TodoItem>
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


// Test Cases
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_empty_list() {
        // Initialize the runtime state with an empty list
        let runtime_state = RuntimeState::default();

        // Test the get function with an empty list
        let todos = get_impl(&runtime_state, 0, 5);

        assert_eq!(todos.len(), 0);
    }
    #[test]
    fn test_get_partial_list() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the get function with a partial list
        let todos = get_impl(&runtime_state, 1, 2);

        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].id, 2);
        assert_eq!(todos[1].id, 3);
    }

    #[test]
    fn test_get_full_list() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the get function with the full list
        let todos = get_impl(&runtime_state, 0, 5);

        assert_eq!(todos.len(), 3);
        assert_eq!(todos[0].id, 1);
        assert_eq!(todos[1].id, 2);
        assert_eq!(todos[2].id, 3);
    }
    
    #[test]
    fn test_get_by_id_existing() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the get_by_id function for an existing item
        let item_id_to_retrieve = 2;
        let retrieved_item = get_by_id_impl(item_id_to_retrieve, &runtime_state);
        
        // Verify that the item was retrieved successfully
        assert!(retrieved_item.is_some());
        let retrieved_item = retrieved_item.unwrap();
        assert_eq!(retrieved_item.id, item_id_to_retrieve);
        assert_eq!(retrieved_item.name, "Todo 2");
        assert_eq!(retrieved_item.done, false);
    }

    #[test]
    fn test_get_by_id_nonexistent() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the get_by_id function for a nonexistent item
        let item_id_to_retrieve = 4;
        let retrieved_item = get_by_id_impl(item_id_to_retrieve, &runtime_state);
        
        // Verify that no item was retrieved
        assert!(retrieved_item.is_none());
    }
    #[test]
    fn test_add_todo() {
        // Initialize the runtime state with an empty list
        let mut runtime_state = RuntimeState::default();

        // Test the add function
        let new_todo_id = add_impl("New Todo".to_string(), &mut runtime_state);

        // Verify that the new TodoItem was added successfully
        assert_eq!(new_todo_id, 1);  // Assuming it's the first item added
        assert_eq!(runtime_state.data.todos.len(), 1);

        let added_todo = &runtime_state.data.todos[0];
        assert_eq!(added_todo.id, new_todo_id);
        assert_eq!(added_todo.name, "New Todo");
        assert_eq!(added_todo.done, false);
    }
    #[test]
    fn test_update_todo_existing() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the update function for an existing item
        let item_id_to_update = 2;
        let new_name = "Updated Todo 2".to_string();
        let is_updated = update_impl(item_id_to_update, new_name.clone(), &mut runtime_state);

        // Verify that the item was updated successfully
        assert!(is_updated);

        let updated_todo = runtime_state.data.todos.iter().find(|i| i.id == item_id_to_update).unwrap();
        assert_eq!(updated_todo.id, item_id_to_update);
        assert_eq!(updated_todo.name, new_name);
        assert_eq!(updated_todo.done, false);
    }
    #[test]
    fn test_update_todo_nonexistent() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the update function for a nonexistent item
        let item_id_to_update = 4;
        let new_name = "Updated Todo 4".to_string();
        let is_updated = update_impl(item_id_to_update, new_name.clone(), &mut runtime_state);

        // Verify that no item was updated
        assert!(!is_updated);
    }
    #[test]
    fn test_delete_todo_existing() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the delete function for an existing item
        let item_id_to_delete = 2;
        let is_deleted = delete_impl(item_id_to_delete, &mut runtime_state);

        // Verify that the item was deleted successfully
        assert!(is_deleted);
        assert_eq!(runtime_state.data.todos.len(), 2);

        // Verify that the deleted item is not present in the list
        let deleted_item = runtime_state.data.todos.iter().find(|i| i.id == item_id_to_delete);
        assert!(deleted_item.is_none());
    }

    #[test]
    fn test_delete_todo_nonexistent() {
        // Initialize the runtime state with some TodoItems
        let mut runtime_state = RuntimeState::default();
        runtime_state.data.todos.push(TodoItem {
            id: 1,
            name: "Todo 1".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 2,
            name: "Todo 2".to_string(),
            done: false,
        });
        runtime_state.data.todos.push(TodoItem {
            id: 3,
            name: "Todo 3".to_string(),
            done: false,
        });

        // Test the delete function for a nonexistent item
        let item_id_to_delete = 4;
        let is_deleted = delete_impl(item_id_to_delete, &mut runtime_state);

        // Verify that no item was deleted
        assert!(!is_deleted);
        assert_eq!(runtime_state.data.todos.len(), 3);
    }
}

