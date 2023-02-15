#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod fs;
use fs::Todo;

#[tauri::command]
fn show_data() -> Result<Todo, String> {
    let todo = Todo::new();

    match todo {
        Ok(todo) => Ok(todo),
        Err(why) => Err(format!("An error has occurred: {}", why)),
    }
}

#[tauri::command]
fn add_todo(key: &str) -> Result<String, String> {
    let todo = Todo::new();

    match todo {
        Ok(mut todo) => {
            todo.insert(key);

            match todo.save() {
                Ok(resp) => Ok(resp),
                Err(why) => Err(format!("An error has occurred: {}", why)),
            }
        }
        Err(why) => Err(format!("An error has occurred: {}", why)),
    }
}

#[tauri::command]
fn update_todo(key: &str) -> Result<String, String> {
    let todo = Todo::new();

    match todo {
        Ok(mut todo) => match todo.complete(key) {
            None => Err(format!("'{}' is not on the to-do list", key.trim())),
            Some(_) => match todo.save() {
                Ok(_) => Ok(format!("Todo '{}' updated", key.trim())),
                Err(why) => Err(format!("An error has occurred: {}", why)),
            },
        },
        Err(why) => Err(format!("An error has occurred: {}", why)),
    }
}

#[tauri::command]
fn remove_todo(key: &str) -> Result<String, String> {
    let todo = Todo::new();

    match todo {
        Ok(mut todo) => match todo.delete(key) {
            None => Err(format!("'{}' is not on the to-do list", key.trim())),
            Some(entry) => match todo.save() {
                Ok(_) => Ok(format!("Todo '{}' deleted", entry)),
                Err(why) => Err(format!("An error has occurred: {}", why)),
            },
        },
        Err(why) => Err(format!("An error has occurred: {}", why)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_data,
            add_todo,
            update_todo,
            remove_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
