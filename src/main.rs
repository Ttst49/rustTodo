use std::io::{Read, stdin};
use std::thread;
use std::time::Duration;

struct TodoList{
    todos: Vec<Todo>
}

impl TodoList {
    fn new()->TodoList{
        TodoList{
            todos:Vec::new()
        }
    }
}

struct Todo{
    title:String,
    id:i64
}

fn init_todo()->TodoList{
    println!("Wait...");
    thread::sleep(Duration::from_secs(2));
    println!("Todolist created!");
    return TodoList::new()
}

fn run(){
    let todolist = init_todo();
    println!("What action to do with your todolist?");
    let mut user_input: String = String::new();
    stdin().read_line(& mut user_input).unwrap();
}


fn main() {
    run()
}