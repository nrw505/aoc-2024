use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    stones: Vec<String>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res = Input { stones: vec![] };

    for line in reader.lines() {
        let string = line.expect("read line");
        let stones: Vec<String> = string.trim().split(' ').map(|e| e.to_string()).collect();

        res.stones.extend(stones);
    }

    res
}

fn blink(stones: &Vec<String>) -> Vec<String> {
    let mut new = vec![];

    for stone in stones {
        let mut new_stones: Vec<String> = vec![];

        if stone == "0" {
            new_stones.push("1".to_string());
        } else if stone.len() % 2 == 0 {
            let i = stone.len() / 2;
            new_stones.push(stone[..i].to_string().parse::<u64>().unwrap().to_string());
            new_stones.push(stone[i..].to_string().parse::<u64>().unwrap().to_string());
        } else {
            let n = stone.parse::<u64>().expect("u64 parse");
            new_stones.push((n * 2024).to_string());
        }

        new.extend(new_stones);
    }

    new
}

fn blink_part2(stones: &HashMap<String, u64>) -> HashMap<String, u64> {
    let mut new = HashMap::new();

    for stone in stones.keys() {
        let count = *(stones.get(stone).unwrap());

        if stone == "0" {
            match new.get("1") {
                Some(n) => new.insert("1".to_string(), n + count),
                None => new.insert("1".to_string(), count),
            };
        } else if stone.len() % 2 == 0 {
            let i = stone.len() / 2;
            let new_left = stone[..i].to_string().parse::<u64>().unwrap().to_string();
            let new_right = stone[i..].to_string().parse::<u64>().unwrap().to_string();

            match new.get(&new_left) {
                Some(n) => new.insert(new_left, n + count),
                None => new.insert(new_left, count),
            };
            match new.get(&new_right) {
                Some(n) => new.insert(new_right, n + count),
                None => new.insert(new_right, count),
            };
        } else {
            let n = stone.parse::<u64>().expect("u64 parse");
            let new_stone = (n * 2024).to_string();

            match new.get(&new_stone) {
                Some(n) => new.insert(new_stone, n + count),
                None => new.insert(new_stone, count),
            };
        };
    }

    new
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let n = 25;
    let mut stones = input.stones;
    for i in 1..(n + 1) {
        stones = blink(&stones);
        // println!("blink {} -> {:?}", i, stones);
    }
    println!("{} stones", stones.len());
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let n = 75;
    let mut stones: HashMap<String, u64> = HashMap::new();

    for stone in input.stones {
        match stones.get(&stone) {
            Some(n) => stones.insert(stone, n + 1),
            None => stones.insert(stone, 1),
        };
    }

    for i in 1..(n + 1) {
        stones = blink_part2(&stones);
        // println!("blink {} -> {:?}", i, stones);
    }
    println!("{} stones", stones.values().sum::<u64>());
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
