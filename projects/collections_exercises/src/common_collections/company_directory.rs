use std::{collections::HashMap, io};

#[derive(Debug)]
pub struct Directory {
    pub directory: HashMap<String, Employee>,
}

#[derive(Debug)]
pub struct Employee {
    pub favorite_pasttime: String,
    pub job: String,
}

pub fn get_command() -> String {
    println!("Would you like to add to a directory or list a directory?");
    println!("Type add, list, or quit.");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line boss");

    return command;
}

pub fn choose_directory() -> String {
    loop {
        println!("Type sales, engineering, or operations");

        let mut directory = String::new();

        io::stdin()
            .read_line(&mut directory)
            .expect("Failed to read line boss");

        match directory.trim().as_ref() {
            "sales" => break String::from("sales"),
            "engineering" => break String::from("engineering"),
            "operations" => break String::from("operations"),
            _ => {
                println!("not a valid command, try again with all lower case and no whitespace");
            }
        }
    }
}

pub fn add_to_directory(directory: &mut Directory) {
    //Init the employee we'd like to add
    println!("Who would you like to add? Type name.");

    let mut employee_name = String::new();

    io::stdin()
        .read_line(&mut employee_name)
        .expect("Failed to read line boss");

    println!("What is their favorite pasttime?");

    let mut pasttime = String::new();

    io::stdin()
        .read_line(&mut pasttime)
        .expect("Failed to read line boss");

    println!("What is their job?");

    let mut job = String::new();

    io::stdin()
        .read_line(&mut job)
        .expect("Failed to read line boss");

    let new_employee = Employee {
        favorite_pasttime: pasttime,
        job,
    };

    directory.directory.insert(employee_name, new_employee);

    println!("{:?}", directory.directory.keys());
    //append employee to the hashmap

    //return the directory
}

pub fn list_directory(directory: &Directory) {
    //sort directory hashmap by employee name, eh too lazy to sort right now
    println!("The directory you requested: {:?}", directory.directory);
    //print sorted directory
}
