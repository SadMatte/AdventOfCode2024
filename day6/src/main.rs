use std::fs;

fn find_guard(layout: &Vec<&str>) -> Vec<usize> {
    for (y_index, _y_plane) in layout.iter().enumerate() {
        for (x_index, x_position) in layout[y_index].chars().enumerate() {
            if x_position == '^' {
                let guard_position: Vec<usize> = vec![x_index, y_index];
                return guard_position;
            }
        }
    }
    panic!("Guard not found");
}

fn guard_move_forward(guard_facing_direction: &str) -> &str {
    match guard_facing_direction {
        "up" => ,
        _ => panic!("Guard facing unknown direction")
    }
}

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

    let input: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    //split the map up into 2 dimensions
    let layout: Vec<&str> = input.split("\n").collect();

    //assumably the direction the guard is facing at the start
    let mut guard_facing_direction: &str = "up";

    let mut guard_position: Vec<usize> = Vec::new();

    guard_position = find_guard(&layout);

    /* 
    //find guard on the 1 dimensional plane
    let guard_1_dimensional_position = if let Some(guard_1_dimensional_position) = input.find('^') {
        guard_1_dimensional_position
    } else {
        panic!("Guard not found");
    };

    println!("{:?}", layout);
    
    
    guard_2_dimensional_position.push(input.get(0).len());
    guard_2_dimensional_position.push(input.len());*/

}
