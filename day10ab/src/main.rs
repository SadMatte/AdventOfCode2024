use std::{collections::HashSet, fs};

//check all possible directions for an increment in height towards a potential max height of 9
fn check_height_for_trail_in_matrix(height_number: &u8, y: &usize, x: &usize, matrix: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {

    println!("{} at {},{}", height_number, x, y);

    let mut peaks_reached: Vec<Vec<usize>> = Vec::new();

    if *height_number == 9 {
        println!();
        return vec![vec![*y, *x]];
    }

    //Checks for out of bound. When an increment is found, keep looking for next increments. This essentially paths itself to all possible 9's and the number returned will be the amount of 9's reached
    if *x != 0 && matrix[*y][*x - 1] == height_number + 1 {
        let found_peaks: Vec<Vec<usize>> = check_height_for_trail_in_matrix(&(height_number+1), y, &(x-1), matrix);
        for found_peak in found_peaks {
            peaks_reached.push(found_peak);
        }
    }
    if *x < matrix[*y].len()-1 && matrix[*y][*x + 1] == height_number + 1 {
        let found_peaks: Vec<Vec<usize>> = check_height_for_trail_in_matrix(&(height_number+1), y, &(x+1), matrix);
        for found_peak in found_peaks {
            peaks_reached.push(found_peak);
        }
    }
    if *y != 0 && matrix[*y - 1][*x] == height_number + 1 {
        let found_peaks: Vec<Vec<usize>> = check_height_for_trail_in_matrix(&(height_number+1), &(y-1), x, matrix);
        for found_peak in found_peaks {
            peaks_reached.push(found_peak);
        }
    }
    if *y < matrix.len()-1 && matrix[*y + 1][*x] == height_number + 1 {
        let found_peaks: Vec<Vec<usize>> = check_height_for_trail_in_matrix(&(height_number+1), &(y+1), x, matrix);
        for found_peak in found_peaks {
            peaks_reached.push(found_peak);
        }
    }

    //remove duplicates before returning
    //COMMENT THESE 2 LINES OUT FOR ANSWER TO PART 2 OF THIS DAY
    let mut unique_peaks_reached: HashSet<Vec<usize>> = HashSet::new();
    peaks_reached.retain(|peak| unique_peaks_reached.insert(peak.clone()));

    peaks_reached
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    //collect entire input into a number matrix
    let matrix: Vec<Vec<u8>> = input.split("\n").map(|s: &str| s.chars().map(|c: char| c as u8 - '0' as u8).collect()).collect();

    println!("{:?}", matrix);

    let mut total: usize = 0;

    for (y_index, y_plane) in matrix.iter().enumerate() {
        for (x_index, height_number) in y_plane.iter().enumerate() {

            if *height_number == 0 {
                let found_peaks = check_height_for_trail_in_matrix(height_number, &y_index, &x_index, &matrix);
                total += found_peaks.len();
            }
        }
    }

    println!("{}", total);
}
