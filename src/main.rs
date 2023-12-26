use std::io;

mod company;

use company::company::*;

fn main() {
    let mut deps: Departments = Departments::new();
    deps.add_department(String::from(ENGINEERING));
    deps.add_department(String::from(SALES));
    deps.add_department(String::from(SUPPORT));

    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();

        match tokens.as_slice() {
            ["Exit"] => break,
            ["All"] => deps.print_all(),
            ["Add", worker, "to", department] => deps.add_worker(department.to_string(), worker.to_string()),
            ["Print", department] => deps.print_department(department.to_string()),
            _ => println!("Your input format is probably wrong..."),
        }
    }
}
