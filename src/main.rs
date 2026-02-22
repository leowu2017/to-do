#[derive(Clone)]
struct Task {
    title: String,
    description: String,
    completed: bool,
}

#[allow(unused)]
fn create_tasks() -> Vec<Task> {
    Vec::<Task>::new()
}

#[allow(unused)]
fn add_task(tasks: &mut Vec<Task>, task: &Task) {
    tasks.push(task.clone());
}

#[allow(unused)]
fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    tasks.remove(index);
}

#[allow(unused)]
fn list_tasks(tasks: &Vec<Task>) {
    for (index, task) in tasks.iter().enumerate() {
        let mark = if task.completed { 'O' } else { 'X' };
        println!("{}. Completed: {}, {}", index, mark, task.title);
        println!("{}", task.description);
        println!();
    }
}

#[allow(unused)]
fn change_task_status(tasks: &mut Vec<Task>, index: usize, completed: bool) {
    tasks[index].completed = completed;
}

fn main() {
    let mut tasks = create_tasks();

    let task0 = Task {
        title: "task1".to_string(),
        description: "This is task1.".to_string(),
        completed: false,
    };
    add_task(&mut tasks, &task0);

    let task1 = Task {
        title: "task2".to_string(),
        description: "This is task2.".to_string(),
        completed: false,
    };
    add_task(&mut tasks, &task1);

    let task2 = Task {
        title: "task3".to_string(),
        description: "This is task3.".to_string(),
        completed: false,
    };
    add_task(&mut tasks, &task2);

    remove_task(&mut tasks, 1);
    change_task_status(&mut tasks, 1, true);

    list_tasks(&tasks);
}
