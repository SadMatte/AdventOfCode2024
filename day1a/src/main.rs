use std::fs;

fn main() {
    //read the file and assign it to the input variable
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");
    //filter the input file into a vector of numbers
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s: &str| s.parse::<i32>().expect("Failed to parse to i32") )
        .collect();
    
    println!("{:?}", numbers);

    //create the 2 distinct lists needed for comparison between values
    let mut left_list: Vec<&i32> = Vec::new();
    let mut right_list: Vec<&i32> = Vec::new();

    //assign the numbers to the lists in the correct order
    for (index, number) in numbers.iter().enumerate() {
        if index % 2 == 0 {
            left_list.push(&number);
        } else {
            right_list.push(&number);
        }
    }

    //sort the lists by numerical value
    left_list.sort();
    right_list.sort();

    let mut total_number_difference: i32 = 0;
    
    //calculate the difference between values between the lists
    for index in 0..left_list.len() {
        let difference: i32 = (left_list[index] - right_list[index]).abs();
        total_number_difference += difference;
    }

    println!("The total number difference between the lists is: {}", total_number_difference);
}
