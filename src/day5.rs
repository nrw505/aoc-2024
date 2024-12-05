#[derive(Debug)]
struct Input {
    rules: Vec<(u64, u64)>,
    updates: Vec<Vec<u64>>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res = Input {
        rules: vec![],
        updates: vec![],
    };

    let mut doing_rules = true;
    for line in reader.lines() {
        let string = line.expect("read line");
        let trimmed = string.trim();

        if doing_rules {
            if trimmed.len() == 0 {
                doing_rules = false;
            } else {
                let parsed: Vec<u64> = trimmed
                    .split("|")
                    .map(|e| e.parse::<u64>().expect("u64 parse"))
                    .collect();
                res.rules.push((parsed[0], parsed[1]));
            }
        } else {
            res.updates.push(
                trimmed
                    .split(",")
                    .map(|e| e.parse::<u64>().expect("u64 parse"))
                    .collect(),
            );
        }
    }

    res
}

fn is_correct_order(update: &Vec<u64>, rules: &Vec<(u64, u64)>) -> bool {
    for rule in rules {
        let (a, b) = rule;

        let ia = update.iter().position(|x| x == a);
        let ib = update.iter().position(|x| x == b);

        match (ia, ib) {
            (Some(c), Some(d)) => {
                if d < c {
                    return false;
                }
            }
            _ => { /* Do nothing */ }
        }
    }
    true
}

fn middle_page(update: &Vec<u64>) -> u64 {
    update[(update.len() - 1) / 2]
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut sum = 0;
    for update in input.updates {
        if is_correct_order(&update, &input.rules) {
            sum += middle_page(&update);
        }
    }
    println!("sum is {}", sum);
}

fn a_comes_before_b(a: u64, b: u64, rules: &Vec<(u64, u64)>) -> std::cmp::Ordering {
    for rule in rules {
        let (x, y) = rule;
        if a == *x && b == *y {
            return std::cmp::Ordering::Less;
        }
        if a == *y && b == *x {
            return std::cmp::Ordering::Greater;
        }
    }
    std::cmp::Ordering::Equal
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);
    let mut sum = 0;
    for mut update in input.updates {
        if !is_correct_order(&update, &input.rules) {
            update.sort_by(|&a, &b| a_comes_before_b(a, b, &input.rules));
            sum += middle_page(&update);
        }
    }
    println!("sum is {}", sum);
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
