use std::io::{stdin};
use std::thread;
use std::time::Duration;
use rand;
use std::num;

const TODO_INDEX: usize = 0;

pub struct TodoList{
    todos: Vec<Todo>
}

impl TodoList {
    pub fn new()->TodoList{
        TodoList{
            todos:Vec::new()
        }
    }

    pub fn create_todo(mut self){
        println!("What title for this todo?");
        let mut new_title = String::new();
        stdin().read_line(&mut new_title).unwrap();
        if new_title.is_empty() {
            new_title = String::from("No title yet...")
        }
        self.todos.insert(TODO_INDEX,Todo::new(new_title));
        println!("{:?}",self.todos)
    }
}

#[derive(Debug)]
pub struct Todo{
    title:String,
    id:i64
}

impl Todo {
    pub fn new(title:String)->Todo{
        Todo{
            title: title.trim().to_string(),
            id:rand::random()
        }
    }
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
        | 4:Show todolist |\n
        | 5:Reset todolist |\n
        --------------------\n
     "
    )
}


fn check_matching(user_input:String,todolist: TodoList){
    if user_input.trim() == String::from("1") {
        TodoList::create_todo(todolist)
    }
}

fn run(){
    let todolist = init_todo();
    println!("What action to do with your todolist?");
    show_menu();
    let mut user_input: String = String::new();
    stdin().read_line(& mut user_input).unwrap();
    check_matching(user_input,todolist);
}


fn main() {
    run()
}