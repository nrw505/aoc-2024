use regex::Regex;

struct Input {
    left: Vec<i64>,
    right: Vec<i64>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res: Input = {
        Input {
            left: vec![],
            right: vec![],
        }
    };

    let re = Regex::new(r"^([0-9]*)\s*([0-9]*)$").expect("regexp parse");
    for line in reader.lines() {
        let string = line.expect("read line");
        let captures = re.captures(&string).expect("regexp match");
        res.left.push(
            captures
                .get(1)
                .expect("regexp capture 1")
                .as_str()
                .parse()
                .expect("integer parse"),
        );
        res.right.push(
            captures
                .get(2)
                .expect("regexp capture 1")
                .as_str()
                .parse()
                .expect("integer parse"),
        );
    }
    res.left.sort();
    res.right.sort();
    res
}

fn day1_part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);
    let zipped = input.left.iter().zip(input.right.iter());

    let mut sum = 0;
    for (_, (l, r)) in zipped.enumerate() {
        let diff = (l - r).abs();

        sum += diff;
        println!("l:{}, r:{}, diff:{}, sum: {}", l, r, diff, sum);
    }
    println!("total is {}", sum)
}

fn day1_part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut sum = 0;
    for (_, l) in input.left.iter().enumerate() {
        let count = input.right.iter().filter(|&r| r == l).count();

        let c = i64::try_from(count).expect("usize -> i64");
        sum += c * (*l);
    }
    println!("total is {}", sum)
}

pub fn day1(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(day1_part1(reader)),
        2 => Ok(day1_part2(reader)),
        _ => todo!(),
    }
}
