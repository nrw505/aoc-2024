#[derive(Debug)]
struct Block {
    fileid: Option<u32>,
}

#[derive(Debug)]
struct Input {
    disk: Vec<Block>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res = Input { disk: vec![] };

    let mut fileid = 0;
    let mut next_is_freespace = false;

    for line in reader.lines() {
        let string = line.expect("line read");
        for c in string.chars() {
            let block_count = c as i32 - '0' as i32;

            let new_block_fileid = if next_is_freespace {
                None
            } else {
                Some(fileid)
            };

            for _ in 0..block_count {
                res.disk.push(Block {
                    fileid: new_block_fileid,
                })
            }

            if next_is_freespace {
                fileid = fileid + 1
            }
            next_is_freespace = !next_is_freespace;
        }
    }

    res
}

fn first_free_block(disk: &Vec<Block>) -> usize {
    disk.iter().position(|block| block.fileid == None).unwrap()
}

fn defrag(disk: &mut Vec<Block>) {
    let mut i = disk.len();
    let mut free_block = first_free_block(disk);

    while free_block < i {
        i = i - 1;

        if disk[i].fileid == None {
            continue;
        }

        disk[free_block].fileid = disk[i].fileid;
        disk[i].fileid = None;

        while disk[free_block].fileid != None {
            free_block = free_block + 1;
        }
    }
}

fn max_file_id(disk: &Vec<Block>) -> u32 {
    disk.iter()
        .filter_map(|e| e.fileid)
        .max()
        .expect("max file id")
}

fn block_indices(disk: &Vec<Block>, fileid: u32) -> Vec<usize> {
    disk.iter()
        .enumerate()
        .filter_map(|(i, block)| {
            if block.fileid == Some(fileid) {
                Some(i)
            } else {
                None
            }
        })
        .collect()
}

fn n_free_blocks_at(disk: &Vec<Block>, index: usize, n: usize) -> bool {
    if n == 0 {
        return true;
    }
    if index + n >= disk.len() {
        return false;
    }

    disk[index].fileid == None && n_free_blocks_at(disk, index + 1, n - 1)
}

fn find_free_space(disk: &Vec<Block>, count: usize) -> Option<usize> {
    for i in 0..disk.len() {
        if n_free_blocks_at(disk, i, count) {
            return Some(i);
        }
    }

    None
}

fn defrag_part2(disk: &mut Vec<Block>) {
    let mut fileid = max_file_id(disk);

    while fileid > 0 {
        let sources = block_indices(disk, fileid);
        let file_size = sources.len();
        let free_space_start = find_free_space(disk, file_size);

        match free_space_start {
            Some(start_block) => {
                if start_block < sources[0] {
                    // Only do this if the first free block is _earlier_ than the file on disk
                    for i in 0..file_size {
                        disk[start_block + i].fileid = Some(fileid);
                        disk[sources[i]].fileid = None;
                    }
                }
            }
            None => { /* Do nothing */ }
        }

        fileid = fileid - 1;
    }
}

fn checksum(disk: &Vec<Block>) -> u64 {
    disk.into_iter()
        .enumerate()
        .map(|(i, block)| match block.fileid {
            Some(fileid) => fileid as u64 * i as u64,
            None => 0,
        })
        .sum()
}

fn part1(reader: impl std::io::BufRead) {
    let mut input = parse_input(reader);

    defrag(&mut input.disk);

    println!("checksum: {}", checksum(&input.disk));
}

fn part2(reader: impl std::io::BufRead) {
    let mut input = parse_input(reader);

    defrag_part2(&mut input.disk);

    println!("checksum: {}", checksum(&input.disk));
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
