use std::fs;
use std::collections::HashMap;

struct Node {
    x: usize,
    y: usize
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    let input: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    let matrix: Vec<&str> = input.split("\n").collect();

    let nodes_map: HashMap<char, Vec<Node>> = HashMap::new();

    for (y_index, y_plane) in matrix.iter().enumerate() {
        for (x_index, node_name) in y_plane.chars().enumerate() {
            if node_name == '.' {
                continue;
            }
            match nodes_map.get(&node_name) {
                Some(nodes) => {
                    nodes.push(Node {y: y_index, x: x_index});
                },
                None => {

                }
            }
        }
    }
}
