mod common_collections;
use common_collections::{pig_latin, vector_statistics};

//mod pig_latin {}
//mod company_directory {}

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
}
