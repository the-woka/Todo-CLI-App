# Todo CLI App

A simple Todo application built with Rust.  
Runs in the terminal, stores data locally, and does exactly what it should.

## Features

- Add a todo
- List all todos
- Mark a todo as done
- Delete a todo
- Persistent local storage

## Build

```bash
cargo build --release
```

## Usages

### Add a todo
```bash
todo add "Activity name"
```

### List todos
```bash
todo list
```

### Mark a todo as done using ID
```bash
todo done 1
```

### Delete a todo using ID
```bash
todo delete 1
```

## Notes
- Todo IDs are auto-incremented
- This app used clap for CLI parsing and chrono for timestamp
