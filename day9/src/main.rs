use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    let mut file_blocks: Vec<i64> = Vec::new();
    let mut data_block_is_free_space: bool = false;
    let mut data_id: usize = 0;

    //create a disk format where '.' is free space and 'i' will always come before a file, to signify its id, and to make long files with large ids possible
    for character in input.chars() {
        for _character_count in 0..(character as u8 - '0' as u8) {
            if data_block_is_free_space {
                file_blocks.push(-1);
            } else {
                file_blocks.push(data_id as i64);
            }
        }
        if !data_block_is_free_space {
            data_id += 1;
        }
        data_block_is_free_space = !data_block_is_free_space;
    }

    println!("{:?}", file_blocks);

    let mut file_blocks_clone: Vec<i64> = file_blocks.clone();
    let mut file_blocks_rearranged: Vec<i64> = Vec::new();

    //re-order so that free space only exists at the end of the disk
    for (file_block_index,file_block) in file_blocks.iter().enumerate() {
        if file_block_index > file_blocks_clone.len()-1 {
            break;
        }
        if *file_block == -1 {
            if let Some(last_file_rev_index) = file_blocks_clone.iter().rev().position(|&file| file >= 0) {
                let last_file_index: usize = file_blocks_clone.len() - 1 - last_file_rev_index;
                file_blocks_rearranged.push(file_blocks_clone[last_file_index]);
                file_blocks_clone.pop();
                while file_blocks_clone[file_blocks_clone.len()-1] as i64 == -1 {
                    file_blocks_clone.pop();
                }
            }
        } else {
            file_blocks_rearranged.push(*file_block);
        }
    }

    println!("{:?}", file_blocks_rearranged);

    let mut total: usize = 0;

    for (index, file_block) in file_blocks_rearranged.iter().enumerate() {
        total += index * (*file_block as usize);
    }

    println!("{}", total);
}
