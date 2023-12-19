pub mod Company {

    use std::collections::HashMap;
    pub enum Department {
        Engineering,
        Sales,
        Support
    }

    pub fn add_employee(
        employee: &str,
        department: Department,
        company: &mut HashMap<String, Department>) {
        
        company.entry(employee.to_string())
            .or_insert(department);
    }

    pub fn sort_employees(company: &HashMap<String, Department>) -> Vec<String> {
        vec!["Hi".to_string()]
    }
}