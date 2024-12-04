use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Mul,
    Do,
    DoNot,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    op: Operation,
    a: u64,
    b: u64,
}

#[derive(Debug)]
struct Input {
    instructions: Vec<Instruction>,
}

fn parse_input(mut reader: impl std::io::BufRead) -> Input {
    let mut res = {
        Input {
            instructions: vec![],
        }
    };

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").expect("regex parse");
    let mut string = String::new();
    reader.read_to_string(&mut string).expect("read file");

    for m in re.captures_iter(&string) {
        let i = Instruction {
            op: Operation::Mul,
            a: m.get(1)
                .expect("match 1")
                .as_str()
                .parse::<u64>()
                .expect("match 1 parse"),
            b: m.get(2)
                .expect("match 2")
                .as_str()
                .parse::<u64>()
                .expect("match 2 parse"),
        };
        res.instructions.push(i);
    }

    res
}

fn parse_input_part2(mut reader: impl std::io::BufRead) -> Input {
    let mut res = {
        Input {
            instructions: vec![],
        }
    };

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").expect("regex parse");
    let mut string = String::new();
    reader.read_to_string(&mut string).expect("read file");

    for m in re.captures_iter(&string) {
        let i = match m.get(0).expect("full match").as_str() {
            "do()" => Instruction {
                op: Operation::Do,
                a: 0,
                b: 0,
            },

            "don't()" => Instruction {
                op: Operation::DoNot,
                a: 0,
                b: 0,
            },

            _ => Instruction {
                op: Operation::Mul,
                a: m.get(1)
                    .expect("match 1")
                    .as_str()
                    .parse::<u64>()
                    .expect("match 1 parse"),
                b: m.get(2)
                    .expect("match 2")
                    .as_str()
                    .parse::<u64>()
                    .expect("match 2 parse"),
            },
        };
        res.instructions.push(i);
    }

    res
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut sum = 0;
    for instruction in input.instructions {
        sum += instruction.a * instruction.b;
    }
    println!("total = {}", sum);
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input_part2(reader);

    let mut sum = 0;
    let mut enabled = true;
    for instruction in input.instructions {
        match instruction.op {
            Operation::Mul => {
                if enabled {
                    sum += instruction.a * instruction.b;
                }
            }
            Operation::Do => {
                enabled = true;
            }
            Operation::DoNot => {
                enabled = false;
            }
        }
    }
    println!("total = {}", sum);
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
