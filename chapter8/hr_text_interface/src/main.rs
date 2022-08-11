use std::{io, collections::HashMap};

enum Action {
    Add(String, String),
    List(String),
    Quit,
    Unknown
}

fn main() {
    let mut map = HashMap::<String, Vec<String>>::new();
    loop {
        let action = next_action();
        match action {
            Action::Add(employee, department) => {
                let employees = map.entry(department).or_insert(Vec::new());
                employees.push(employee);
            },
            Action::List(department) => {
                let employees = map.get_mut(&department);
                if let Some(employees) = employees  {
                    employees.sort_unstable();
                    println!("Department '{}' has employees {:?}", department, employees);
                } else {
                    println!("Department '{}' has has no employees yet", department);
                }
            },
            Action::Quit => {
                break;
            },
            Action::Unknown => {}
        }
    }
}

fn next_action() -> Action {
    let mut input = String::new();
    println!();
    println!("What do you want to do?");
    println!("a $PERSON $DEPARTMENT => adds $PERSON to $DEPARTMENT");
    println!("l $DEPARTMENT         => lists all members of $DEPARTMENT in alphabetic order");
    println!("q                     => quits the app");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input_to_action(input)
}

fn input_to_action(s: String) -> Action {
    let words: Vec<&str> = s.split_whitespace().collect();
    match words.as_slice() {
        [ "a", name, department ] =>  Action::Add(name.to_string(), department.to_string()),
        [ "l", department ] => Action::List(department.to_string()),
        [ "q" ] => Action::Quit,
        _ => Action::Unknown
    }
}

