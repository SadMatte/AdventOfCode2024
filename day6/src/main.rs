use std::fs;

fn find_guard(layout: &Vec<Vec<char>>) -> Vec<usize> {
    for (y_index, _y_plane) in layout.iter().enumerate() {
        for (x_index, x_position) in layout[y_index].iter().enumerate() {
            if x_position == &'^' {
                let guard_position: Vec<usize> = vec![y_index, x_index];
                return guard_position;
            }
        }
    }
    panic!("Guard not found");
}

fn is_object_in_front_of_guard(guard_facing_direction: &str, guard_position: &Vec<usize>, layout: &Vec<Vec<char>>) -> bool {
    match guard_facing_direction {
        "up" => {
            if layout[guard_position[0] - 1][guard_position[1]] == '#' {
                return true;
            }
        },
        "right" => {
            if layout[guard_position[0]][guard_position[1] + 1] == '#' {
                return true;
            }
        },
        "down" => {
            if layout[guard_position[0] + 1][guard_position[1]] == '#' {
                return true;
            }
        },
        "left" => {
            if layout[guard_position[0]][guard_position[1] - 1] == '#' {
                return true;
            }
        },
        _ => panic!("Guard facing unknown direction")
    }
    return false;
}

//moves guard in the direction it's facing
fn guard_move_forward(guard_facing_direction: &str, guard_position: &Vec<usize>) -> Vec<usize> {
    match guard_facing_direction {
        "up" => return vec![guard_position[0] - 1, guard_position[1]],
        "right" => return vec![guard_position[0], guard_position[1] + 1],
        "down" => return vec![guard_position[0] + 1, guard_position[1]],
        "left" => return vec![guard_position[0], guard_position[1] - 1],
        _ => panic!("Guard facing unknown direction")
    }
}

//turns guard to the right of whereever it's facing
fn guard_turn_right(guard_facing_direction: &str) -> &str {
    match guard_facing_direction {
        "up" => return "right",
        "right" => return "down",
        "down" => return "left",
        "left" => return "up",
        _ => panic!("Guard facing unknown direction")
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    //split the map up into 2 dimensions
    let mut layout: Vec<Vec<char>> = input
        .split("\n")
        .map(|line: &str| line.chars().collect())
        .collect();

    //assumably the direction the guard is facing at the start
    let mut guard_facing_direction: &str = "up";

    let mut guard_position: Vec<usize> = find_guard(&layout);

    //set start as patrolled by guard
    layout[guard_position[0]][guard_position[1]] = 'X';

    //loop until guard is out of bounds
    while guard_position[0] < layout.len()-1 && guard_position[0] > 0 && guard_position[1] < layout[0].len()-1 && guard_position[1] > 0 {

        if is_object_in_front_of_guard(guard_facing_direction, &guard_position, &layout) {
            guard_facing_direction = guard_turn_right(guard_facing_direction);
        }
        else {
            guard_position = guard_move_forward(guard_facing_direction, &guard_position);
            layout[guard_position[0]][guard_position[1]] = 'X';
        }
    }

    //check how many X's are in the map (spaces the guard has walked)
    let mut total_marked_spaces: i32 = 0;
    for y_plane in &layout {
        for mark in y_plane {
            if mark == &'X' {
                total_marked_spaces += 1;
            }
        }
        println!("{:?}", y_plane);
    }

    println!("{}", total_marked_spaces);

}
