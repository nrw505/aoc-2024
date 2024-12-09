#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

#[derive(Debug)]
struct Input {
    equations: Vec<Equation>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res = Input { equations: vec![] };

    for line in reader.lines() {
        let parts: Vec<String> = line
            .expect("read line")
            .trim()
            .split(":")
            .map(|e| e.to_string())
            .collect();
        let result = parts[0].parse::<u64>().expect("u64 parse");
        let numbers = parts[1]
            .trim()
            .split(" ")
            .map(|e| e.parse::<u64>().expect("u64 parse"))
            .collect();

        res.equations.push(Equation {
            result: result,
            numbers: numbers,
        });
    }

    res
}

fn calculate_possibilities(numbers: &Vec<u64>) -> Vec<u64> {
    numbers.iter().fold(vec![] as Vec<u64>, |acc, &x| {
        if acc.len() == 0 {
            vec![x]
        } else {
            acc.into_iter().flat_map(|e| vec![e + x, e * x]).collect()
        }
    })
}

fn is_possible(eqn: &Equation) -> bool {
    let mut possibilities: Vec<u64> = vec![];

    possibilities = calculate_possibilities(&eqn.numbers);
    possibilities.contains(&eqn.result)
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut sum = 0;
    for eqn in input.equations {
        if is_possible(&eqn) {
            sum = sum + eqn.result;
        }
    }
    println!("sum is {}", sum);
}

fn conc(a: u64, b: u64) -> u64 {
    (a.to_string() + &b.to_string())
        .parse::<u64>()
        .expect("u64 parse")
}

fn calculate_possibilities_part2(numbers: &Vec<u64>) -> Vec<u64> {
    numbers.iter().fold(vec![] as Vec<u64>, |acc, &x| {
        if acc.len() == 0 {
            vec![x]
        } else {
            acc.into_iter()
                .flat_map(|e| vec![e + x, e * x, conc(e, x)])
                .collect()
        }
    })
}

fn is_possible_part2(eqn: &Equation) -> bool {
    let mut possibilities: Vec<u64> = vec![];

    possibilities = calculate_possibilities_part2(&eqn.numbers);
    possibilities.contains(&eqn.result)
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut sum = 0;
    for eqn in input.equations {
        if is_possible_part2(&eqn) {
            sum = sum + eqn.result;
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
