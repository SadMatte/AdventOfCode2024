use std::fs;

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut stones_after_blink: Vec<u64> = Vec::new();
    for stone in stones {
        let stone_as_string: String = stone.to_string();
        if *stone == 0 {
            stones_after_blink.push(1);
        }
        else if stone_as_string.len() % 2 == 0 {
            let first_half_stone: u64 = stone_as_string.get(0..stone_as_string.len()/2).expect("Invalid range in string").parse::<u64>().expect("Type conversion failed");
            let second_half_stone: u64 = stone_as_string.get(stone_as_string.len()/2..).expect("Invalid range in string").parse::<u64>().expect("Type conversion failed");
            stones_after_blink.push(first_half_stone);
            stones_after_blink.push(second_half_stone);
        }
        else {
            stones_after_blink.push(*stone * 2024);
        }
    }
    stones_after_blink
}

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s: &str| s.parse::<u64>().expect("Type conversion failed"))
        .collect();
    
    for n in 0..25 {
        stones = blink(&stones);
        println!("{n}");
    }

    //println!("{:?}", stones);
    println!("There are {} stones", stones.len());
}
