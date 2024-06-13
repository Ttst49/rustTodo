use std::fmt::Error;
use std::io::{stdin};
use std::thread;
use std::time::Duration;
use postgres::{Client, NoTls};

struct TodoList{
    todos: Vec<Todo>
}

impl TodoList {
     fn new()->TodoList{
        TodoList{
            todos:Vec::new()
        }
    }

     fn create_todo(&mut self, mut todo_index:usize){
        println!("What title for this todo?");
        let mut new_title = String::new();
        stdin().read_line(&mut new_title).unwrap();
        if new_title.is_empty() {
            new_title = String::from("No title yet...")
        }
        self.todos.insert(todo_index,Todo::new(new_title,todo_index));
         todo_index+=1;
        run(self,todo_index);
    }

     fn remove_todo(&mut self,todo_index:usize){
         show_actual_todo(self,false,todo_index);
         let mut todo_id = String::new();
         stdin().read_line(&mut todo_id).unwrap();
         Todo::remove(self, todo_id);
         run(self,todo_index)
    }

    fn edit_todo(&mut self,todo_index:usize){
        show_actual_todo(self,false,todo_index);
        println!("Which one to edit?");
        let mut todo_id = String::new();
        stdin().read_line(&mut todo_id).unwrap();
        Todo::edit(self,todo_id);
        run(self,todo_index)
    }

    fn reset_todolist(&mut self, todo_index:usize){
        println!("you sure? (y/n)");
        let mut user_answer = String::new();
        stdin().read_line(&mut user_answer).unwrap();
        if
                user_answer.trim() == "y"
                    || user_answer.trim() == "yes"
                    || user_answer.trim() == "Yes"
        {
            println!("clearing...");
            self.todos.clear();
        }else {
            println!("Cancelling")
        }
        run(self,todo_index)
    }
}

 struct Todo{
    title:String,
    id:usize
}

impl Todo {
    fn new(title:String,todo_index:usize)->Todo{
        Todo{
            title: title.trim().to_string(),
            id:todo_index
        }
    }
    fn remove(todolist: &mut TodoList, todo_id: String){
        println!("Which one to remove?");
        let todo_id_parsed:usize = todo_id.trim().parse().unwrap();
        if let Some(index) = todolist.todos.iter().position(|n| n.id == todo_id_parsed){
            todolist.todos.remove(index);
            println!("removed the todo!")
        }else {
            println!("Invalid value")
        }
    }

    fn edit(todolist: &mut TodoList, todo_id:String){
        let todo_id_parsed:usize = todo_id.trim().parse().unwrap();
        if let Some(index) = todolist.todos.iter().position(|n| n.id == todo_id_parsed){
            println!("What's the new title?");
            let mut new_title = String::new();
            stdin().read_line(&mut new_title).unwrap();
            if !&new_title.is_empty() {
                todolist.todos[index].title = new_title;
                println!("your todo has been edited!");
            }else {
                println!("Invalid value")
            }
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
        | 3:Edit a todo    |\n
        | 4:Show todolist  |\n
        | 5:Reset todolist |\n
        | 6:Quit           |\n
        --------------------\n
     "
    )
}

fn show_actual_todo(todolist: &mut TodoList,from_menu:bool,todo_index:usize){
    for todo in &todolist.todos {
        println!("---------------------");
        println!("title:{}\n",todo.title);
        println!("id:{} \n",todo.id);
        println!("---------------------");
    }
    if from_menu {
        thread::sleep(Duration::from_secs(2));
        run(todolist,todo_index);
    }
}


fn check_matching(user_input:String, todolist: &mut TodoList, todo_index:usize){

    match user_input.trim() {
        "1"=>{TodoList::create_todo(todolist, todo_index)},
        "2"=>{TodoList::remove_todo(todolist,todo_index)},
        "3"=>{TodoList::edit_todo(todolist,todo_index)},
        "4"=>{show_actual_todo(todolist,true,todo_index)},
        "5"=>{TodoList::reset_todolist(todolist,todo_index)},
        "6"=>{println!("See ya")},
        _=>{
            println!("You have to give a valid input!");
            run(todolist,todo_index)
        }
    }
}

fn run(todolist: &mut TodoList, todo_index:usize){
    println!("What action to do with your todolist?");
    show_menu();
    let mut user_input: String = String::new();
    stdin().read_line(& mut user_input).unwrap();
    check_matching(user_input,todolist,todo_index);
}


fn main() {

    connect_to_database();
    //let todo_index: usize = 0;
    //let mut todolist = init_todo();
    //run(&mut todolist,todo_index)
}

fn connect_to_database() -> Result<(), postgres::Error> {
    let mut client
        = Client::connect("postgresql://todorustuser:postgres@localhost/todorust", NoTls)?;
    let test = client.batch_execute("SELECT * FROM todo");
   println!("{}", test.iter().count());
    test
}