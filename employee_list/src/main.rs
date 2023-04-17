use regex::Regex;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut directory = HashMap::<String, Vec<String>>::new();

    let add_cmd = Regex::new(r"^[Aa]dd (?P<name>[[:alpha:]]+) to (?P<department>[[:alpha:]]+)").unwrap();
    let exit_cmd = Regex::new(r"^[Ee]xit").unwrap();
    let list_cmd = Regex::new(r"^[Ll]ist (?P<department>[[:alpha:]]+)").unwrap();

    loop {
        println!("enter command:");
        let cmd = {
            let mut cmd = String::new();
            io::stdin().read_line(&mut cmd).unwrap();
            cmd
        };

        if add_cmd.is_match(&cmd) {
            let captures = add_cmd.captures(&cmd).unwrap();
            let employee = captures.name("name").unwrap().as_str();
            let department = captures.name("department").unwrap().as_str();

            directory
                .entry(department.to_string())
                .or_insert(Vec::<String>::new());

            let mut employee_list = directory.get_mut(department).unwrap();

            employee_list.push(employee.to_string());
            employee_list.sort();
            employee_list.dedup();
        } else if list_cmd.is_match(&cmd) {
            let captures = list_cmd.captures(&cmd).unwrap();
            if let Some(department) = captures.name("department") {
                let department = department.as_str();
                if let Some(employee_list) = directory.get(department) {
                    println!("{}:", department);
                    for employee in employee_list.iter() {
                        println!("    {employee}");
                    }
                } else {
                    for (department, employee_list) in &directory {
                        println!("{}:", department);
                        for employee in employee_list.iter() {
                            println!("    {employee}");
                        }
                    }
                }
            }
        } else if exit_cmd.is_match(&cmd) {
            break;
        } else {
            println!("unknown command: {cmd}");
            continue;
        }
    }
    println!("exiting...");
}
