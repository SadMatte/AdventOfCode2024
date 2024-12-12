use std::fs;

fn main() {
    //read the file and assign it to the input variable
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");
    //filter the input file into a vector of numbers
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|s: &str| s.parse::<usize>().expect("Failed to parse to i32") )
        .collect();
    
    println!("{:?}", numbers);

    //create the 2 distinct lists needed for comparison between values
    let mut left_list: Vec<usize> = Vec::new();
    let mut right_list: Vec<usize> = Vec::new();

    //assign the numbers to the lists in the correct order
    for (index, number) in numbers.iter().enumerate() {
        if index % 2 == 0 {
            left_list.push(*number);
        } else {
            right_list.push(*number);
        }
    }

    //sort the lists by numerical value
    left_list.sort();
    right_list.sort();

    let mut total_similarity_score: usize = 0;
    
    //calculate similarity score
    for index in 0..left_list.len() {
        let score: usize = left_list[index] * right_list.iter().filter(|&&x| x == left_list[index]).count();
        total_similarity_score += score;
    }

    println!("The total similarity score is: {}", total_similarity_score);
}
