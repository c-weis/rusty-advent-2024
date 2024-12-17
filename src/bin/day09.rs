use std::cmp;

use itertools::Itertools;
use rusty_advent_2024::utils::lines_from_file;

#[derive(Clone, Copy, Debug)]
enum DataBlock {
    File { id: usize, size: usize },
    Free { size: usize },
}

fn partial_checksum(id: usize, start_position: usize, size: usize) -> u128 {
    (id * (start_position..start_position + size).sum::<usize>()) as u128
}

fn checksum(harddisk: &Vec<DataBlock>) -> u128 {
    let mut checksum: u128 = 0;
    let mut seeker: usize = 0;

    for block in harddisk {
        match block {
            DataBlock::Free { size } => {
                seeker += *size;
            }
            DataBlock::File { id, size } => {
                checksum += partial_checksum(*id, seeker, *size);
                seeker += *size;
            }
        }
    }

    checksum
}

fn compressed(harddisk: &Vec<DataBlock>) -> Vec<DataBlock> {
    // Part 1: right uber_block only ever has one component in it
    let mut left_block_idx = 0;
    let mut right_block_idx = &harddisk.len() - 1;
    let mut compressed_harddisk: Vec<DataBlock> = Vec::new();

    let mut free_space_in_left_block: Option<usize> = None;
    let mut files_remaining_in_right_block: Option<usize> = None;
    while left_block_idx < right_block_idx {
        let (left_block, right_block) = (&harddisk[left_block_idx], &harddisk[right_block_idx]);

        match (left_block, right_block) {
            (_, DataBlock::Free { size: _ }) => right_block_idx -= 1,
            (DataBlock::File { id, size }, _) => {
                compressed_harddisk.push(DataBlock::File {
                    id: *id,
                    size: *size,
                });
                left_block_idx += 1;
            }
            (
                DataBlock::Free { size: free_size },
                DataBlock::File {
                    id: file_id,
                    size: file_size,
                },
            ) => {
                let free_size = match free_space_in_left_block {
                    Some(free_size_left) => free_size_left,
                    None => *free_size,
                };
                let file_size = match files_remaining_in_right_block {
                    Some(file_size_right) => file_size_right,
                    None => *file_size,
                };

                let movable_files = cmp::min(free_size, file_size);
                let (new_free_size, new_file_size) =
                    (free_size - movable_files, file_size - movable_files);

                compressed_harddisk.push(DataBlock::File {
                    id: *file_id,
                    size: movable_files,
                });

                if new_free_size == 0 {
                    left_block_idx += 1;
                    free_space_in_left_block = None;
                } else {
                    free_space_in_left_block = Some(new_free_size);
                }

                if new_file_size == 0 {
                    right_block_idx -= 1;
                    files_remaining_in_right_block = None;
                } else {
                    files_remaining_in_right_block = Some(new_file_size);
                }
            }
        }
    }

    if let Some(size_left) = files_remaining_in_right_block {
        if let DataBlock::File { id, size: _ } = &harddisk[right_block_idx] {
            compressed_harddisk.push(DataBlock::File {
                id: *id,
                size: size_left,
            })
        }
    } else if let DataBlock::File { id, size } = &harddisk[left_block_idx] {
        compressed_harddisk.push(DataBlock::File {
            id: *id,
            size: *size,
        });
    }

    compressed_harddisk
}

fn blocks_from_string(string: String) -> Vec<DataBlock> {
    string
        .split("")
        .filter_map(|character| -> Option<usize> { character.parse().ok() })
        .enumerate()
        .map(|(idx, size)| -> DataBlock {
            if idx % 2 == 0 {
                DataBlock::File { id: idx / 2, size }
            } else {
                DataBlock::Free { size }
            }
        })
        .collect_vec()
}

fn main() {
    println!("Answer to part 1:");
    println!("{}", part1("input/input09.txt"));
    println!("Answer to part 2:");
    println!("{}", part2("input/input09.txt"));
}

fn part1(path: &str) -> u128 {
    let string = lines_from_file(path)
        .map(|line| line.unwrap())
        .find_or_first(|_| true)
        .expect("No input found.");

    let blocks = blocks_from_string(string);

    let compressed_blocks = compressed(&blocks);

    checksum(&compressed_blocks)
}

fn part2(_path: &str) -> u128 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_checksum() {
        assert!(partial_checksum(7, 10, 5) == 7 * (10 + 11 + 12 + 13 + 14))
    }

    #[test]
    fn test_tiny_disks() {
        // "2": 00 -> 00
        let hdd1 = compressed(&blocks_from_string(String::from("2")));
        assert!(checksum(&hdd1) == 0);

        // "232": 00...11 -> 0011...
        let hdd2 = compressed(&blocks_from_string(String::from("232")));
        assert!(checksum(&hdd2) == 5);

        // "12345": 0..111....22222 -> 022111222.....
        let hdd3 = compressed(&blocks_from_string(String::from("12345")));
        assert!(
            checksum(&hdd3)
                == (partial_checksum(0, 0, 1)
                    + partial_checksum(2, 1, 2)
                    + partial_checksum(1, 3, 3)
                    + partial_checksum(2, 6, 3)) as u128
        );

        // "3132": 000.111.. -> 000111...
        let hdd4 = compressed(&blocks_from_string(String::from("3132")));
        assert!(checksum(&hdd4) == 3 + 4 + 5);
    }

    #[test]
    fn test_part1() {
        assert!(part1("input/input09.txt.test1") == 1928);
    }

    #[test]
    fn test_part2() {
        assert!(part2("input/input09.txt.test1") == 0);
    }
}
