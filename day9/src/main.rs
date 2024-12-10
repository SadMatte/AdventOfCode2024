use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt")
        .expect("File not found");

    let input: &str = "2333133121414131402";

    let mut file_blocks: String = String::new();
    let mut data_block_is_free_space: bool = false;
    let mut data_id: usize = 0;

    //create a disk format where '.' is free space and 'i' will always come before a file, to signify its id, and to make long files with large ids possible
    for character in input.chars() {
        for _character_count in 0..(character as u8 - '0' as u8) {
            if data_block_is_free_space {
                file_blocks.push('.');
            } else {
                file_blocks.push('i');
                let data_id_str: String = data_id.to_string();
                for data_id_char in data_id_str.chars() {
                    file_blocks.push(data_id_char);
                }
            }
        }
        if !data_block_is_free_space {
            data_id += 1;
        }
        data_block_is_free_space = !data_block_is_free_space;
    }

    println!("{}", file_blocks);

    let mut file_blocks_clone: String = file_blocks.clone();

    //re-order so that free space only exists at the end of the disk
    for (character_index, character) in file_blocks.chars().enumerate() {
        if character == '.' {
            //find range of last file and swap its location with the first occurence of '.'
            if let Some(last_i_index) = file_blocks_clone.rfind('i') {
                let slice_from_i: &str = &file_blocks_clone[last_i_index..];
                if let Some(dot_index) = slice_from_i.find('.') {
                    let end_index: usize = last_i_index + dot_index;
                    file_blocks_clone
                }
            }
            file_blocks_clone.replace_range(character_index..character_index+1, replace_with);
        }
    }
}
