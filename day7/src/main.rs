use std::fs;

fn has_all_operational_possibilties_been_tried(operational_possibilties: &Vec<bool>) -> bool {
    for operator in operational_possibilties {
        //if operator is plus, then not all possibilities have been tried
        if operator == &false {
            return false;
        }
    }
    true
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    //divide into seperate expressions
    let expressions: Vec<&str> = input.split("\n").collect();

    //prepare variables to catch values
    let mut results: Vec<u128> = Vec::new();
    let mut numbers: Vec<Vec<u128>> = Vec::new();

    let mut total: u128 = 0;

    //go through every expression
    for (expression_index, expression) in expressions.iter().enumerate() {
        let mut split_expression = expression.split(": ");

        //convert split_expression from Option<&str> to &str, then from &str to u128 and assign to results
        if let Some(result_string) = split_expression.next() {
            if let Ok(result) = result_string.parse::<u128>() {
                results.push(result);
            } else {
                println!("Failed to parse \"{}\" as u64", result_string);
            }
        }

        //convert the second part of split_expression from Option<&str> to &str, then from &str to split u128 to assign to numbers
        if let Some(numbers_string) = split_expression.next() {
            let number_vec: Vec<u128> = numbers_string
                .split_whitespace()
                .filter_map(|s: &str| s.parse::<u128>().ok())
                .collect();
            numbers.push(number_vec);
        }
        
        //create a vector where every value represents an operator between every number; false is + and true is *
        let mut operational_possibilties: Vec<bool> = vec![false; numbers[expression_index].len()];
        let mut operational_possibilties_loop_container: Vec<bool> = vec![false; numbers[expression_index].len()];
        loop {

            let mut expression_result: u128 = 0;
            for (operator_index, operator) in operational_possibilties.iter().enumerate() {
                
                //if operator is *
                if *operator {
                    expression_result *= numbers[expression_index][operator_index];
                }
                //operator is +
                else {
                    expression_result += numbers[expression_index][operator_index];
                }
            }
            if results[expression_index] == expression_result && /* bandaid solution to the first operator not always being plus, or the starting value of expression_result not being the first number in the expression */ !operational_possibilties[0] {
                println!("{} is a correct expression with:\n{:?}", expression, operational_possibilties);
                total += expression_result;
                break;
            }

            //iterate over all possible combinations of operators in a binary kind of way
            for (operator_index, operator) in operational_possibilties.iter().enumerate() {

                //if operator is *
                if *operator {
                    operational_possibilties_loop_container[operator_index] = false;
                }
                //operator is +
                else {
                    operational_possibilties_loop_container[operator_index] = true;
                    break;
                }
            }
            operational_possibilties = operational_possibilties_loop_container.clone();

            if has_all_operational_possibilties_been_tried(&operational_possibilties) {
                break;
            }
        }
    }

    println!("total value of all correct expressions: {}", total);

}
