use crate::task::Task;
use std::collections::HashMap;

pub struct TaskList {
    _tasks: HashMap<String, Task>,
}
impl TaskList {
    pub fn add_task(mut task_list: TaskList, task: String) -> () {
        task_list
            ._tasks
            .insert(task.clone(), Task::new_task(task.clone()));
    }

    pub fn complete_task(mut task_list: TaskList, task: String) -> () {
        Task::complete_task(task_list._tasks.get(task.clone()));
    }

    pub fn remove_task(mut task_list: TaskList, task: String) -> () {
        task_list._tasks.remove(task);
    }
}
