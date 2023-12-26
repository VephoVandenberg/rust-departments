use std::{io, collections::HashMap};

mod company;

use company::CompanyAndDepartments::*;

fn main() {

    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    deps.insert(String::from(ENGINEERING), Vec::new());
    deps.insert(String::from(SALES), Vec::new());
    deps.insert(String::from(SUPPORT), Vec::new());

    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        match tokens.as_slice() {
            ["Exit"] => break,
            ["All"] => print_all(&deps),
            ["Add", worker, "to", department] => add_person(department.to_string(), worker.to_string(), &mut deps),
            ["Print", department] => print_department(department.to_string(), &mut deps),
            _ => println!("Your input format is probably wrong..."),
        }
    }
}

fn print_department(department: String, deps: &mut HashMap<String, Vec<String>>) {
    if deps.contains_key(&department){
        deps.get_mut(&department).unwrap().sort();

        for worker in deps.get(&department).unwrap() {
            println!("{worker}");
        }

    } else {
        println!("-------------------");
        println!("No such department.");
        println!("-------------------");
    }
}

fn add_person(
    department: String,
    name: String, 
    deps: &mut HashMap<String, Vec<String>>) {

        if deps.contains_key(&department) {
            deps.get_mut(&department).unwrap().push(name);
        }
}

fn print_all(deps: &HashMap<String, Vec<String>>) {
    for (name, dep) in deps.iter() {
        println!("-------------------");
        println!("{name}");
        println!("-------------------");
        for worker in dep {
            println!("{worker}");
        }
    }
}
