use std::collections::HashMap;

#[derive(Clone)]
pub struct Company {
    employee_map: HashMap<String, Vec<String>>
}

trait PersonnelOrganizer {
    fn add_personnel(&mut self, new_employee: String);
    fn employees_by_dept(&self, department: String) -> Option<&Vec<String>>;
    fn all_employees(&self) -> Option<Vec<String>>;
}

impl PersonnelOrganizer for Company {
    fn add_personnel(&mut self, new_employee: String) {
        // parse input
        let department = new_employee.split_whitespace().nth(3).unwrap().to_string();
        let name = new_employee.split_whitespace().nth(1).unwrap().to_string();
        // Add to map
        self.employee_map
                .entry(department)
                .or_default()
                .push(name);
    }
    fn employees_by_dept(&self, department: String) -> Option<&Vec<String>> {
        self.employee_map.get(&department)
    }
    fn all_employees(&self) -> Option<Vec<String>> {
        let mut result = vec![];
        for (_, val) in self.employee_map.iter() {
            result.append(&mut val.clone());
        }
        result.sort();
        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = 2 + 2;
        // assert_eq!(result, 4);
        let mut target = Company {employee_map: HashMap::new()};
        target.add_personnel(String::from("ADD Andrew to Engineering"));
        target.add_personnel(String::from("ADD Elena to Engineering"));
        let ok1 = vec!["Andrew".to_string(), "Elena".to_string()];
        match target.employees_by_dept(String::from("Engineering")) {
            Some(v) => assert_eq!(ok1, *v),
            None => assert_eq!(1, 0)
        }

        target.add_personnel(String::from("ADD Sally to Science"));
        target.add_personnel(String::from("ADD Jake to Science"));
        let ok2 = vec!["Andrew", "Elena", "Jake", "Sally"];
        match target.all_employees() {
            Some(v) => assert_eq!(ok2, v),
            None => assert_eq!(1, 0)
        }
    }
}
