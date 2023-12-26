pub mod company {
    use std::collections::HashMap;

    pub const ENGINEERING: &str = "Engineering";
    pub const SALES: &str = "Sales";
    pub const SUPPORT: &str = "Support";

    type Worker = String;
    type Department = String;

    pub struct Departments {
        departments: HashMap<Department, Vec<Worker>>
    }

    impl Departments {
        pub fn new() -> Departments {
            Departments { departments:  HashMap::new() }
        }

        pub fn add_department(&mut self, department: Department) {
            self.departments
                .entry(department)
                .or_insert(vec![]);
        }

        pub fn add_worker(&mut self, department: Department, worker: Worker) {
            if self.departments.contains_key(&department) {
                self.departments
                    .get_mut(&department)
                    .unwrap()
                    .push(worker);
            } else {
                println!("-------------------");
                println!("No such department.");
                println!("-------------------");
            }
        }

        pub fn print_department(&mut self, department: Department) {
            if self.departments.contains_key(&department){
                self.departments.get_mut(&department).unwrap().sort();
                for worker in self.departments.get_mut(&department).unwrap() {
                    println!("{worker}");
                }
        
            } else {
                println!("-------------------");
                println!("No such department.");
                println!("-------------------");
            }
        }

        pub fn print_all(&self) {
            for (name, dep) in self.departments.iter() {
                println!("-------------------");
                println!("{name}");
                println!("-------------------");
                for worker in dep {
                    println!("{worker}");
                }
            }
        }
    }
}
