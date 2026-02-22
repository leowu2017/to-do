# Rust To-Do List Practice Exercises

This is a set of To-Do List practice exercises. Each step adds one new functionality and reinforces Rust concepts: ownership, borrowing, error handling, traits, generics, concurrency, and more.

---

## 1. Tasks and in-memory list

* Define a `Task` struct (e.g. title, description, completed).
* Store tasks in a `Vec<Task>` and implement **add_task** and **list_tasks** (ownership/borrowing as you go).

## 2. Remove tasks

* Implement **remove_task** (by index or title), using references and mutable access to the list.

## 3. Mark tasks complete

* Implement **mark_task_as_completed** (and optionally “uncomplete”) by borrowing mutably and updating a task’s status (e.g. by title or id).

## 4. Validation and errors

* Introduce **validation**: no duplicate tasks, and operations only on existing tasks.
* Use `Result` and custom error types (`std::error::Error`) for “task not found” and “duplicate task”; use `?` for propagation.

## 5. Formatted display (traits)

* Define a **Displayable** (or similar) trait and implement it for `Task` to support formatted output (e.g. pretty-printed list).

## 6. Generic task list

* Make the list **generic** over the item type and add trait bounds so you can have different kinds of “tasks” (e.g. different structs) in the same list.

## 7. Priority and sorting

* Add **priority** (and optionally due date) to tasks and implement **sorting** (by priority, date, or both).

## 8. Command-line interface

* Add a **CLI** using `clap` (or `structopt`) so users can add, remove, list, and mark tasks from the shell.

## 9. Save and load to file

* **Persistence**: serialize the task list to JSON with `serde`, write/read a file, and handle file I/O and format errors.

## 10. Auto-save

* **Auto-save**: after each add/remove/update, write the list to the file so the app always reflects the latest state on disk.

## 11. Concurrency

* **Concurrent usage**: shared list behind `Arc<Mutex<>>`, and either multi-threaded access (`std::thread`) or async add/update with `async`/`await`.

---

Each step delivers one clear new capability and builds on the previous ones, suitable for intermediate Rust practice.
