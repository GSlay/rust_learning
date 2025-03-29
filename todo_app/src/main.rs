mod task;
use crate::task_list::TaskList;
use std::io;
mod task_list;

const task_list: TaskList = TaskList::new();

fn main() {
    let mut input = String::new();
    while input != "exit" {
        io::stdin()
            .read_line(&mut input)
            .expect("failed read_line()");
    }
}

fn input_to_task(input: String) -> () {}
