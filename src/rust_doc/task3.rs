use std::collections::HashMap;

#[derive(Debug)]
struct Company {
    // employees: Vec<String>,
    asign_map: HashMap<String, String>,
}

impl Company {
    pub fn new() -> Self {
        Company {
            // employees: Vec::new(),
            asign_map: HashMap::new(),
        }
    }

    fn add_employee(&mut self, name: &str, department: &str) {
        // self.employees.push(name.to_string());
        self.asign_map
            .insert(name.to_string(), department.to_string());
    }

    fn print_company_emps(&self) {
        for (name, dep) in self.asign_map.iter() {
            println!("{name} works in {dep}");
        }
    }
}

pub fn run_task() {
    let mut company = Company::new();

    company.add_employee("worker1", "dep1");
    company.add_employee("worker2", "dep2");
    company.add_employee("worker3", "dep2");
    company.add_employee("worker4", "dep3");

    company.print_company_emps();

    println!("{company:#?}")
}
