use std::fs;


fn evaluate_full_instruction(full_instruction: &Vec<i32>, rule_set: &Vec<Vec<i32>> ) -> bool {
    for (index_of_first_instruction, _instruction) in full_instruction.iter().enumerate() {

        let higher_order_number: i32 = full_instruction[index_of_first_instruction];

        //compare what is supposed to be the higher ordered number to all the numbers supposedly below it, if any combination of numbers violate this rule, then declare this whole instruction invalid and move on to the next one
        for index_of_second_instruction in index_of_first_instruction+1..full_instruction.len() {
            let lower_order_number: i32 = full_instruction[index_of_second_instruction];

            if !is_valid_order_of_numbers(higher_order_number, lower_order_number, rule_set) {
                return false;
            }
        }
    }
    return true;
}

fn is_valid_order_of_numbers(higher_order_number: i32, lower_order_number: i32, rule_set: &Vec<Vec<i32>>) -> bool {
    for rule in rule_set {
        if rule == &vec![higher_order_number, lower_order_number] {
            println!("{} is correctly higher than {}", higher_order_number, lower_order_number);
            return true;
        }
    }
    println!("{} is incorrectly higher than {}", higher_order_number, lower_order_number);
    return false;
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    //split the input into 2 parts: the rules and instructions
    let parts: Vec<&str> = input.split("\n\n").collect();

    println!("{:?}", parts);

    //split the stringified rules into a list of stringified rules
    let rules: Vec<&str> = parts[0].split("\n").collect();

    let mut rule_set: Vec<Vec<i32>> = Vec::new();
    /*create the ruleset where each rule is defined by 2 numbers
    example:
    [
        [47, 53],
        [97, 13],
        [97, 61],
        ...
    ]
    */
    for rule in rules {
        rule_set.push(rule
            .split("|")
            .map(|stringified_number| stringified_number.parse::<i32>().expect("Could not convert"))
            .collect());
    }

    println!("{:?}", rule_set);

    //split the stringified instructions into a list of stringified instructions
    let instructions: Vec<&str> = parts[1].split("\n").collect();

    let mut instruction_set: Vec<Vec<i32>> = Vec::new();
    /*create the instruction set where each instruction is defined by any amount of numbers
    example:
    [
        [75, 47, 61, 53, 29],
        [97, 61, 53, 29, 13],
        [75, 29, 13],
        ...
    ]
    */
    for instruction in instructions {
        instruction_set.push(instruction
            .split(",")
            .map(|stringified_number| stringified_number.parse::<i32>().expect("Could not convert"))
            .collect());
    }

    println!("{:?}", instruction_set);


    let mut total: i32 = 0;

    //loop through all instructions in the instruction set
    for full_instruction in instruction_set {
        if evaluate_full_instruction(&full_instruction, &rule_set) {
            total += full_instruction[(full_instruction.len()-1)/2];
        }
    }

    println!("{total}");

}
