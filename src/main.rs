use std::io::{stdin};
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

fn show_menu(){
    println!(
        "\n
        --------------------\n
        | 1:Add a todo     |\n
        | 2:Remove a todo  |\n
        | 3:Modify a todo  |\n
        | 4:Reset todolist |\n
        --------------------"
    )
}

fn run(){
    let todolist = init_todo();
    println!("What action to do with your todolist?");
    let mut user_input: String = String::new();
    stdin().read_line(& mut user_input).unwrap();
    show_menu()
}


fn main() {
    run()
}