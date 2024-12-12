use std::fs;

//first checks if report seems to be incrementing or decrementing, then if at any point it does the opposite, then deem this report false. Also deem false if increment/decrement is greater than 3 or stays unchanging
fn check_report(report: &Vec<u8>) -> usize {

    let is_incrementing_level: bool = report[0] < report[1];

    let mut last_level: i16 = report[0] as i16;

    for index in 1..report.len() {
        let level: i16 = report[index] as i16;
        let level_differential: u8 = (level - last_level).abs() as u8;
        if level_differential == 0 || level_differential > 3 || (is_incrementing_level && level < last_level) || (!is_incrementing_level && level > last_level) {
            return 0;
        }
        last_level = level;
    }

    println!("{:?}", report);
    return 1;
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("file not found");

    let reports: Vec<Vec<u8>> = input
        .split("\n")
        .map(|line| {
            line.split_whitespace()
            .map(|num| num.parse::<u8>().expect("Invalid number"))
            .collect()
        })
        .collect();

    let mut total: usize = 0;

    //go through all reports
    for report in reports {
        total += check_report(&report);
    }
    
    println!("{}", total);
}
