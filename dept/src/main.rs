use std::collections::HashMap;
use std::io;

fn main() {

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\nMenu:");
        println!("1. Add an employee to a department");
        println!("2. Retrieve all employees in a department");
        println!("3. Retrieve all employess in the company");
        println!("4. Exit");
        println!("5. Enter your choice");
        
        let mut choice = String::new();
        
        io::stdin.read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();
        
        match choice {
            "1" => add_emp(&mut company),
            "2" => list_dept(&company),
            "3" => list_emp(&company),
            "4" => {
                println!("Exiting...");
                break
            }
            _ => println!("Invalid choice. Please try again");
        }
    

    }

    

}

fn add_emp(company: &mut HashMap<String, Vec<String>>) {

    //print to enter department name
    //create a department variable
    //create the input placeholder
    //the the input placeholder to holder your variable
    // trim and covert it to string
    //add it to the HashMap

    println!("Enter a department");
    
    let mut department = String::new();
    
    io::stdin().read_line(&mut department).expect("Failed to department");
    let department = department.trim().to_string();
    
    println!("Enter employee name");
    let mut emp = String::new();
    io::stdin().read_line(&mut emp).expect("Failed to read emp");
    let emp = emp.trim().to_string();
    
    company.entry(department.clone()).or_insert_with(Vec::new).push(emp.clone());
    
    println!("Added {} to {}", emp,department);
}

fn list_dept(company: &HashMap<String, Vec<String>>) {
    println!("Enter departments name");
    
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("Failed to read input");
    let department = department.trim().to_string();
    
    //match company to get department
    //create a Some emp variable to match => sorted emp
    
    if let Some(emp) = company.get(&department) {
        let mut sorted_emp = emp.clone();
        sorted_emp.sort();
        
        println!("Employes in {}: {:?}", department, sorted_emp);
    } else {
        println!("Department {} not found.", department);
    }
    
}

fn list_emp(company: &HashMap<String, Vec<String>>) {
    if company.is_empty() {
        println!("No departmentrs found");
    }
    
    for (department,emp) in company {
        let mut sort_emp = emp.clone();
        sort_emp.sort();
        println!("{}: {:?}", department, sort_emp);
    }
}
