const WELCOME_MESSAGE : &str = "Hallo";
const GOODBYE_MESSAGE : &str = "Bye";

//Type Todo
struct Todo {
    name : String,
    completed : bool,
}

//Type TodoList
struct TodoList {
   pub list : Vec<Todo>,
}
static mut Todo_list: Vec<Todo> = Vec::new();

fn main() {
    //make todolist
    start(); 
}

fn start(){
    

    let action = ask_for_action();
    match action {
        UserAction::Add => add_todo(),
        UserAction::List => list_todos(),
        UserAction::Exit => exit(),
        UserAction::Retry => ask_again(),
        UserAction::Delete => delete_todo(),
        UserAction::Complete => complete_todo()
    }
}
fn add_todo(){
    println!("Enter a todo item :");
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("no of bytes read , {}", b1);
    let todo_item = line.trim();
    println!("todo item {}", todo_item);
    let todo = Todo {
        name : String::from(todo_item),
        completed : false,
    };
    unsafe {
        Todo_list.push(todo);
    }
    start();
}
fn list_todos(){
    println!("Todo list :");
    println!("===========");
    unsafe {
        for (i, item) in Todo_list.iter().enumerate() {
            println!("{} [{}] - {}", i, item.completed, item.name);
        }
    }
    start();
} 
fn exit(){ 
    println!("Goodbye!");
    std::process::exit(0);
} 
fn ask_again(){ 
        println!("Sorry, I didn't understand that command.");
    start();
}
fn delete_todo(){
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("no of bytes read , {}", b1);
    let choice_num : usize = line.trim().parse().unwrap();
    println!("choice num {}", choice_num);
    unsafe {
        Todo_list.remove(choice_num);
    }
    start();

}
fn complete_todo(){
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("no of bytes read , {}", b1);
    let choice_num : usize = line.trim().parse().unwrap();
    println!("choice num {}", choice_num);
    unsafe {
        Todo_list[choice_num].completed = true;
    }
    start();
}

enum UserAction {
    Add,
    Delete,
    List,
    Complete,
    Exit,
    Retry,
}

fn ask_for_action() -> UserAction {
    println!("What do you want to do?");
    println!("1. Add");
    println!("2. Delete");
    println!("3. List");
    println!("4. Complete");
    println!("5. Exit");
    println!("Enter your choice :");
    let mut line = String::new();
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("no of bytes read , {}", b1);
    let choice_num : i32 = line.trim().parse().unwrap();
    println!("choice num {}", choice_num);
    let choice = match choice_num {
        1 => UserAction::Add,
        2 => UserAction::Delete,
        3 => UserAction::List,
        4 => UserAction::Complete,
        5 => UserAction::Exit,
        _ => UserAction::Retry
    };

    return choice;
}
