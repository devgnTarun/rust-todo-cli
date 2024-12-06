
# Weilder - A CLI Todo App Built with Rust

## Overview

**Weilder** is a simple yet simple (haha) command-line todo app built with **Rust**. 
In this project, you can:
- Add tasks
- Edit tasks
- Delete tasks
- View tasks

Let's get you started with installing, building, and running this powerful CLI tool.

---

## Requirements

Before using **CLI TOOL**, you'll need to have **Rust** installed on your machine. If you haven’t installed it yet, follow these simple steps:

1. Install Rust from [rust-lang.org](https://www.rust-lang.org/).
2. Ensure you have `cargo` and `rustc` available in your terminal.

---

## Installation

To install and run **CLI TOOL**, follow these steps:

### 1. Clone the Repository

Clone this repository to your local machine.

```
cd weilder
```

### 2. Build the Project

After cloning, build the project using Cargo:

For Debug mode:
```bash
cargo build
```

For Release mode (optimized version):
```bash
cargo build --release
```

---

## Running the Project

### Running in Debug Mode

Once the project is built in Debug mode, you can run the executable directly from the command line.

- On **Windows**:
  ```bash
  ./target/debug/weilder.exe
  ./target/debug/cli-todo-rust.exe
  ```

- On **Linux/Mac**:
  ```bash
  ./target/debug/weilder
  Or
  ./target/debug/cli-todo-rust
  ```

### Running in Release Mode

Once the project is built in Release mode, the binary will be optimized for better performance. You can find the executable in the `target/release` directory.

- On **Windows**:
  ```bash
  ./target/release/weilder.exe
  ./target/release/cli-todo-rust.exe
  ```

- On **Linux/Mac**:
  ```bash
  ./target/release/weilder
  ./target/release/cli-todo-rust
  ```

---

## Available Commands

Here are the commands you can use with **Weilder**:

### 1. **Add a Task**

To add a new task to your list:

```bash
weilder add --title "Task Title" --desc "Task Description"
```

**Options**:
- `--title` : The title of the task.
- `--desc` : The description of the task.

Example:
```bash
weilder add --title "Finish Rust Project" --desc "Complete the Weilder app."
```

### 2. **Edit a Task**

To edit an existing task by providing its `id` and the new details:

```bash
weilder edit --id 1 --title "New Title" --desc "Updated description" --completed true
```

**Options**:
- `--id` : The ID of the task you want to edit.
- `--title` : New title for the task (optional).
- `--desc` : New description for the task (optional).
- `--completed` : Mark the task as completed (`true`/`false`) (optional).

Example:
```bash
weilder edit --id 1 --title "Complete Rust App" --desc "Fix bugs and improve functionality" --completed true
```

### 3. **Delete a Task**

To delete a task by its ID:

```bash
weilder delete --id 1
```

**Options**:
- `--id` : The ID of the task you want to delete.

Example:
```bash
weilder delete --id 1
```

### 4. **View All Tasks**

To view all the tasks you’ve created, simply use:

```bash
weilder list
```

This will display all tasks, along with their titles, descriptions, and completion status.

---

## Executable Usage After Build

Once you've built the project, you can use the following commands after running the `.exe` (Windows) or binary (Linux/Mac):

### On Windows:

```bash
./target/debug/weilder.exe add --title "My Task" --desc "Description"
```

or if you are running the release build:

```bash
./target/release/weilder.exe add --title "My Task" --desc "Description"
```

### On Linux/Mac:

```bash
./target/debug/weilder add --title "My Task" --desc "Description"
```

or if you are running the release build:

```bash
./target/release/weilder add --title "My Task" --desc "Description"
```

---

## Troubleshooting

- **Error: "Could not load the module"**  
  If you see an error saying "The module './target' could not be loaded", make sure you are running the command correctly from the terminal. Ensure you are in the project directory.

- **Can't find executable?**  
  If you can’t find the executable after building the project, double-check the paths mentioned in the `target/debug` or `target/release` directories.

---
