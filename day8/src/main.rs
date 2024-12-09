use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

struct Node {
    x: i64,
    y: i64
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct AntiNode {
    x: i64,
    y: i64
}

fn remove_duplicates(anti_nodes: &mut Vec<AntiNode>) {
    let mut unique_anti_nodes = HashSet::new();
    anti_nodes.retain(|anti_node| unique_anti_nodes.insert(anti_node.clone()));
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    /*
    ......#....#
    ...#....0...
    ....#0....#.
    ..#....0....
    ....0....#..
    .#....A.....
    ...#........
    #......#....
    ........A...
    .........A..
    ..........#.
    ..........#.
    */

    //matrix of all chars
    let matrix: Vec<&str> = input.split("\n").collect();

    //create maps to keep track of nodes
    let mut nodes_map: HashMap<char, Vec<Node>> = HashMap::new();
    let mut anti_nodes_map: Vec<AntiNode> = Vec::new();

    for (y_index, y_plane) in matrix.iter().enumerate() {
        for (x_index, node_name) in y_plane.chars().enumerate() {
            if node_name == '.' {
                continue;
            }

            nodes_map.entry(node_name).or_insert(Vec::new()).push(Node {
                y: y_index as i64,
                x: x_index as i64
            });
        }
    }

    //go through all nodes' following nodes and if there is already an antinode there or it's out of bounds, then don't add it
    for (node_name, nodes) in &nodes_map {
        for (first_node_index, _node) in nodes.iter().enumerate() {
            for second_node_index in first_node_index+1..nodes.len() {
                println!("first y of {} number {} is {}", node_name, first_node_index, nodes[first_node_index].y);
                println!("first x of {} number {} is {}", node_name, first_node_index, nodes[first_node_index].x);
                println!("second y of {} number {} is {}", node_name, second_node_index, nodes[second_node_index].y);
                println!("second x of {} number {} is {}", node_name, second_node_index, nodes[second_node_index].x);
                let first_anti_node: AntiNode = AntiNode {
                    y: nodes[first_node_index].y*2 - nodes[second_node_index].y,
                    x: nodes[first_node_index].x*2 - nodes[second_node_index].x
                };
                let second_anti_node: AntiNode = AntiNode {
                    y: nodes[second_node_index].y*2 - nodes[first_node_index].y,
                    x: nodes[second_node_index].x*2 - nodes[first_node_index].x
                };

                //if antinode is out of bounds then don't add
                if first_anti_node.y < matrix.len() as i64 && first_anti_node.x < matrix[0].len() as i64 && first_anti_node.y >= 0 && first_anti_node.x >= 0 {
                    println!("Antinode accepted with position {}, {}\n", first_anti_node.x, first_anti_node.y);
                    anti_nodes_map.push(first_anti_node);
                }
                if second_anti_node.y < matrix.len() as i64 && second_anti_node.x < matrix[0].len() as i64 && second_anti_node.y >= 0 && second_anti_node.x >= 0 {
                    println!("Antinode accepted with position {}, {}\n", second_anti_node.x, second_anti_node.y);
                    anti_nodes_map.push(second_anti_node);
                }
            }
        }
    }

    //remove duplicates that may have been made
    remove_duplicates(&mut anti_nodes_map);

    //count anti nodes
    let mut total_anti_nodes: usize = 0;
    total_anti_nodes += anti_nodes_map.len();
    
    println!("There are a total of {} anti nodes", total_anti_nodes);
}
