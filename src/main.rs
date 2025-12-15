mod storage;
mod todo;

use ::chrono::Utc;
use clap::{Parser, Subcommand};
use storage::{load_todos, save_todos};
use todo::Todo;

#[derive(Parser)]
#[command(name = "todo")]
#[command(version = "0.1.0")]
#[command(about = "Todo CLI App")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { text: String },

    List,

    Done { id: u32 },

    Delete { id: u32 },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text } => {
            let mut todos = load_todos();

            let new_id: u32 = if todos.is_empty() {
                1
            } else {
                todos.iter().map(|t| t.id).max().unwrap() + 1
            };

            let new_todo = Todo {
                id: new_id,
                text: text.clone(),
                completed: false,
                created_at: Utc::now(),
            };

            todos.push(new_todo);

            save_todos(&todos);

            println!("Added {}", text);
        }

        Commands::List => {
            let todos = load_todos();

            if todos.is_empty() {
                println!("No todos yet. Use 'todo add \"text\"' to add one.");
                return;
            }

            println!("\n Your Todos: ");
            println!("-----------------");

            for todo in &todos {
                let status = if todo.completed { "x" } else { " " };
                println!("{}. [{}] {}", todo.id, status, todo.text);
            }

            println!("-----------------");

            let pending = todos.iter().filter(|t| !t.completed).count();
            let done = todos.iter().filter(|t| t.completed).count();

            println!("Total: {} pending, {} completed", pending, done);
        }

        Commands::Done { id } => {
            let mut todos = load_todos();

            let todo = todos.iter_mut().find(|t| t.id == id);

            match todo {
                Some(todo) => {
                    todo.completed = true;
                    let text = todo.text.clone();

                    save_todos(&todos);
                    println!("Marked as done: {}", text);
                }
                None => {
                    println!("Todo with ID {} not found", id);
                }
            }
        }

        Commands::Delete { id } => {
            let mut todos = load_todos();

            let index = todos.iter().position(|t| t.id == id);

            match index {
                Some(index) => {
                    let removed = todos.remove(index);
                    save_todos(&todos);
                    println!("Deleted: {}", removed.text);
                }
                None => {
                    println!("Todo with ID {} not found", id);
                }
            }
        }
    }
}
