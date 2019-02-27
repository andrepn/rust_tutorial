mod common_collections;
use common_collections::{company_directory, pig_latin, vector_statistics};
use std::collections::HashMap;

fn main() {
    //PROBLEM 1
    //Given a list of integers, use a vector and return the mean
    //(the average value), median (when sorted, the value in the
    //middle position), and mode (the value that occurs most often;
    //a hash map will be helpful here) of the list.

    //Declare vector to get statistics, could allow input
    let mut list_of_numbers = vec![1, 5, 5, 5, 9];

    //Calculate mean
    let mean_of_numbers = vector_statistics::mean(&list_of_numbers);
    println!("The mean of the list is {}", mean_of_numbers);

    //Calculate mode
    let mode_of_numbers = vector_statistics::mode(&list_of_numbers);
    println!("The modes of the list are {:?}", mode_of_numbers);

    //Calculate median
    let median_of_numbers = vector_statistics::median(&mut list_of_numbers);
    println!("The median of the list is {}", median_of_numbers);

    //PROBLEM 2
    //Convert strings to pig latin. The first consonant of each word is
    //moved to the end of the word and “ay” is added, so “first” becomes
    //“irst-fay.” Words that start with a vowel have “hay” added to the
    //end instead (“apple” becomes “apple-hay”). Keep in mind the details
    //about UTF-8 encoding!

    let normal_string = String::from("That rug really tied the room together");
    let pig_latin_string = pig_latin::pig_latinize(&normal_string);
    println!("The original string was {}", normal_string);
    println!("Now in pig latin: {}", pig_latin_string);

    //PROBLEM 3
    //Using a hash map and vectors, create a text interface to allow a
    //user to add employee names to a department in a company. For
    //example, “Add Sally to Engineering” or “Add Amir to Sales.” Then
    //let the user retrieve a list of all people in a department or all
    //people in the company by department, sorted alphabetically.
    println!("Welcome to megacorp boss!");

    use company_directory::*;

    let mut sales_directory = Directory {
        directory: HashMap::new(),
    };

    let mut engineering_directory = Directory {
        directory: HashMap::new(),
    };

    let mut operations_directory = Directory {
        directory: HashMap::new(),
    };

    loop {
        let command = company_directory::get_command();

        match command.trim().as_ref() {
            "add" => {
                println!("Alright let's add to a directory. Choose which directory to add in:");
                let directory = choose_directory();

                match directory.as_ref() {
                    "sales" => {
                        add_to_directory(&mut sales_directory);
                    }

                    "operations" => {
                        add_to_directory(&mut operations_directory);
                    }

                    "engineering" => {
                        add_to_directory(&mut engineering_directory);
                    }

                    &_ => println!("Doh!"),
                }
            }
            "list" => {
                println!("Alright choose a directory to list:");
                let directory = choose_directory();

                match directory.as_ref() {
                    "sales" => {
                        list_directory(&sales_directory);
                    }

                    "operations" => {
                        list_directory(&operations_directory);
                    }

                    "engineering" => {
                        list_directory(&engineering_directory);
                    }
                    &_ => println!("Doh!"),
                }
            }
            "quit" => {
                println!("Until next time boss");
                break;
            }
            _ => {
                println!("not a valid command, try again with all lower case and no whitespace");
            }
        }
    }
}
