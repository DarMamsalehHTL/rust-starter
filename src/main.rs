use std::io;
fn main() {
    println!("What do you want to do? (1) add task, (2) view tasks, (3) mark task as done, (4) delete task");
    let read: String = io::stdin().read_line(&mut String::new()).unwrap().to_string();
    let read: i32 = read.trim().parse().unwrap();
    if read == 1
    {
        add_task();
    }
}

fn add_task() {
    println!("Enter task name:");
    let _read = io::stdin().read_line(&mut String::new()).unwrap();
    

}
