use std::io;

enum Priority{
    Low,
    Medium,
    High,
}

struct Task {
    name:String,
    completed:bool,
    priority:Priority
}

fn main() {
    println!("Hello world!, Welcome to To-Do List Logger ");
    let mut task :Vec<Task> = Vec::new();

    loop{
        println!("Please select the operation ");
        println!("\n 1. Add Task");
        println!("\n 2. View Task");
        println!("\n 3. Mark Task Done");
        println!("\n 4. Delete Task");
        println!("\n 5. Quit");
        let mut x = String::new();
        io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");

        let x:i32 = match x.trim().parse(){
            Ok(num) => num,
            Err(..) => {
                println!("Please enter valid number");
                continue;
            }
        };

        match x {
            1 => add_task(&mut task),
            2 => view_task(&mut task),
            3 => mark_task_done(&mut task),
            4 => delete_task(&mut task),
            5 => {
                println!("See You Soon !");
                break;
            }
            _ => println!("Pleae eter valid choice"),
        }; 
    }
}

fn add_task(tasks :&mut Vec<Task>){
    let mut name = String::new();
    let mut priority = String::new();
    
    println!("Please enter task name ");
    io::stdin()
    .read_line(&mut name)
    .expect("Failed to read Line");
     let  name = name.trim().to_string();

    println!("Please select Priority of complition");
    io::stdin()
    .read_line(&mut priority)
    .expect("Failed to read Line");
     
     let priority =  match priority.trim() {
        "1" => Priority::High,
        "2" => Priority::Medium,
        "3" => Priority::Low,
         _  => {
            println!("please enter valid priority");
            return;
        }
     };

    

     let task= Task{
        name,
        completed:false,
        priority,
     };

     tasks.push(task);

}

fn  view_task(tasks :&mut Vec<Task>) {
    
    if tasks.is_empty() {
        println!("Nothing to display, please add the task!");
        return;
    }

    let mut count:i32 = 1;

    for task in tasks {
        println!("{}.",count);
        println!("Task name :{}",task.name);
        println!("Status :{}",task.completed);
        println!("Priority :{}",
                    match task.priority {
                       Priority::Low => "Low",
                       Priority::Medium => "Medium",
                       Priority::High => "High", 
                    }
                );
        count = count+1;

    }
    
}

fn mark_task_done(tasks :&mut Vec<Task>) {

    if tasks.is_empty() {
        println!("Nothing to display, please add the task!");
        return;
    }

    println!("Please Enter task which you want to change status : ");

    let mut task_name = String::new();
    io::stdin()
    .read_line(&mut task_name)
    .expect("Failed to reas Line");

    let task_name = task_name.trim().to_string();

    let mut found = false;

    for task in tasks.iter_mut(){
        if task.name == task_name {
            task.completed = true;
            found = true;
            println!("Task {} marked as completed",task.name);
            break;
        }
    }

    if !found 
    {
        println!("Task {} didn't found, Please enter a correct task",task_name);
    }
}

fn delete_task(tasks :&mut Vec<Task>) {

    if tasks.is_empty() {
        println!("Nothing to display, please add the task!");
        return;
    }

    println!("Please enter the task name :");

    let mut task_name = String::new();
    io::stdin()
    .read_line(&mut task_name)
    .expect("Failed to reas Line");

    let task_name = task_name.trim().to_string();


    let leg = tasks.len();

    tasks.retain(|task| task.name != task_name);

    if leg > tasks.len()
    {
        println!("Task {} deleted successfully",task_name);
    }else {
        println!("Task {} didn't found . Please enter correct task name !",task_name);
    }

}






