advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let mut empty_space = Vec::new();
    let mut files = Vec::new();
    let mut disk_size = 0;
    let mut file_id = 0;
    for (index, c) in input.chars().enumerate() {
        let digit = char::to_digit(c, 10).unwrap() as u64;
        let is_empty_block = index % 2 == 0;
        match is_empty_block {
            true => {
                files.extend((disk_size..disk_size+digit).map(|file| (file_id, file)));
                file_id += 1;
            },
            false => empty_space.extend(disk_size..disk_size+digit),
        }
        disk_size += digit;
    }

    let mut pointer = files.len() - 1;
    for index in &empty_space {
        let current_file = &files[pointer];
        if index > &current_file.1 {
            break;
        }
        files[pointer] = (current_file.0, *index);
        pointer -= 1;
    }

    let mut checksum = 0;
    for file in &files {
        checksum += file.0 * file.1;
    }
    Some(checksum)
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct FileBlock {
    size: u64,
    id: u64,
    start_index: u64,
}

impl FileBlock {
    fn new(file_size: u64, file_id: u64, start_index: u64) -> Self {
        FileBlock {
            size: file_size,
            id: file_id,
            start_index,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct EmptyBlock {
    size: u64,
    start_index: u64,
}

impl EmptyBlock {
    fn new(empty_block_size: u64, start_index: u64) -> Self {
        EmptyBlock {
            size: empty_block_size,
            start_index,
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut empty_space = Vec::new();
    let mut files = Vec::new();
    let mut disk_size = 0;
    let mut file_id = 0;
    for (index, c) in input.chars().enumerate() {
        let digit = char::to_digit(c, 10).unwrap() as u64;
        let is_empty_block = index % 2 == 0;
        match is_empty_block {
            true => {
                files.push(FileBlock::new(digit, file_id, disk_size));
                file_id += 1;
            },
            false => empty_space.push(EmptyBlock::new(digit, disk_size)),
        }
        disk_size += digit;
    }

    for file in files.iter_mut().rev() {
        if let Some(empty_block) = empty_space.iter_mut()
            .find(|block| block.start_index <= file.start_index && block.size >= file.size)
        {
            file.start_index = empty_block.start_index;

            empty_block.start_index += file.size;
            empty_block.size -= file.size;
            if empty_block.size == 0 {
                empty_space.retain(|block| block.size > 0);
            }
        }
    }

    let mut checksum = 0;
    for file in &files {
        for i in 0..file.size {
            checksum += file.id * (file.start_index + i)
        }
    }
    Some(checksum as u64)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
