use crate::models::task::Task;
use serde_json::{from_reader, to_writer_pretty};
use std::fs::{File, OpenOptions};

pub fn add_task(title: String, desc: String) {
    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("task.json")
        .expect("Failed to open task.json");

    let mut tasks: Vec<Task> = match from_reader(&file) {
        Ok(exisiting_task) => exisiting_task,
        Err(_) => Vec::new(),
    };

    let next_id = tasks.last().map_or(1, |task| task.id + 1);

    let new_task = Task {
        id: next_id,
        title,
        description: desc,
        completed: false,
    };
    tasks.push(new_task);

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("task.json")
        .expect("Failed to open a task.json");

    to_writer_pretty(file, &tasks).expect("Failed to write the task!");

    println!("Task Added Successfully!");
}

pub fn list_tasks() {
    let file = File::open("task.json").expect("Failed to open task.json");

    let tasks: Vec<Task> = match from_reader(&file) {
        Ok(existing_task) => existing_task,
        Err(_) => {
            print!("Task not found");
            return;
        }
    };

    print!("Your Tasks");

    for task in tasks {
        println!(
            "ID: {}, Title: {}, Description: {}, Completed: {}",
            task.id, task.title, task.description, task.completed
        );
    }
}

pub fn edit_tasks(id: u32, title: String, desc: String, completed: Option<bool>) {
    let file: File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("task.json")
        .expect("Failed to open the task.json");

    let mut tasks: Vec<Task> = match from_reader(&file) {
        Ok(exisiting_task) => exisiting_task,
        Err(_) => {
            println!("No Tasks found!");
            return;
        }
    };

    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        if !title.is_empty() {
            task.title = title;
        }
        if !desc.is_empty() {
            task.description = desc;
        }
        if let Some(c) = completed {
            task.completed = c;
        }

        let file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open("task.json")
            .expect("Failed to open the task.json");

        to_writer_pretty(file, &tasks).expect("Failed to write the task");
        println!("Task updated successfully!");
    } else {
        println!("Task with ID {} not found.", id);
    };
}

pub fn delete_task(id: u32) {
    let file: File = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open("task.json")
        .expect("Unable to open the Task.json.");

    let mut tasks: Vec<Task> = match from_reader(&file) {
        Ok(exisiting_task) => exisiting_task,
        Err(_) => {
            println!("No Tasks found!");
            return;
        }
    };

    let initial_length = tasks.len();

    tasks.retain(|task| task.id != id);

    // If no task was deleted, notify the user
    if tasks.len() == initial_length {
        println!("Task with ID {} not found. Nothing was deleted.", id);
        return;
    }

    // Write the updated tasks back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("task.json")
        .expect("Unable to write to the Task.json.");

    to_writer_pretty(&mut file, &tasks).expect("Failed to write tasks to the file.");

    // Substitute to use not to open the file again and truncate anything

    // -----  we can do set_len(0) and remove all the stuffs from file and then
    // --------- seek(SeekFrom::Start(0)), rewrite the file from the begining!

    // file.set_len(0).expect("Failed to truncate the file.");
    // file.seek(SeekFrom::Start(0)).expect("Failed to reset file cursor.");

    // // Write the updated data
    // to_writer_pretty(&file, &tasks).expect("Failed to write tasks to the file.");
    println!("Task with ID {} has been deleted successfully.", id);
}
