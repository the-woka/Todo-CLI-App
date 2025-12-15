use crate::todo::Todo;
use std::path::Path;
use serde_json;

const TODOS_FILE : &str = "todos.json";

pub fn load_todos() -> Vec<Todo> {
    if !Path::new(TODOS_FILE).exists() {
        return Vec::new();
    }

    let data = match std::fs::read_to_string(TODOS_FILE){
        Ok(content) => content,
        Err(_) => return Vec::new(),
    };

    match serde_json::from_str::<Vec<Todo>>(&data) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    }
}

pub fn save_todos(todos: &Vec<Todo>){
    let json_string = serde_json::to_string_pretty(todos)
        .expect("Failed to serialize todos");
    std::fs::write(TODOS_FILE, json_string)
        .expect("Failed to write data");
}