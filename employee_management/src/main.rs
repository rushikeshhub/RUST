// Rust Packages
use std::io;
use std::collections::HashMap;

/// Entry point of the application
fn main() {
    // HashMap to store department -> employees list
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Hello, Welcome to the Employee Management Program!");

    // Loop until user chooses to quit
    loop {
        println!("\nPlease select a task to perform:");
        println!("1. Add Employee Details");
        println!("2. View Specific Department Details");
        println!("3. View All Department Details");
        println!("4. Quit");

        // Take user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse input into integer
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        // Match input to actions
        match input {
            1 => add_employee(&mut company),
            2 => view_department(&mut company),
            3 => view_all_departments(&mut company),
            4 => {
                println!("👋 See you soon!");
                break;
            }
            _ => println!("❌ Invalid option, please try again."),
        }
    }
}

/// Adds a new employee to a department
fn add_employee(map: &mut HashMap<String, Vec<String>>) {
    println!("Enter department name:");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read line");
    let department = department.trim().to_string();

    println!("Enter employee name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    map.entry(department).or_insert(Vec::new()).push(name);
    println!("✅ Employee added successfully.");
}

/// Views employees in a specific department
fn view_department(map: &HashMap<String, Vec<String>>) {
    if map.is_empty() {
        println!("⚠️ No departments added yet.");
        return;
    }

    println!("Enter department name:");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read line");
    let department = department.trim().to_string();

    match map.get(&department) {
        Some(employees) => {
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("\n📋 {} Department Employees:", department);
            for emp in sorted_employees {
                println!(" - {}", emp);
            }
        }
        None => println!("❌ No such department found."),
    }
}

/// Views all departments and their employees
fn view_all_departments(map: &HashMap<String, Vec<String>>) {
    if map.is_empty() {
        println!("⚠️ No data to display.");
        return;
    }

    println!("\n📊 All Departments and Employees:");
    for (department, employees) in map {
        let mut sorted_employees = employees.clone();
        sorted_employees.sort();
        println!("\n{} Department:", department);
        for emp in sorted_employees {
            println!(" - {}", emp);
        }
    }
}
