#[derive(Debug)]
struct Report {
    levels: Vec<i64>,
}

#[derive(Debug)]
struct Input {
    reports: Vec<Report>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    let mut res = { Input { reports: vec![] } };

    for line in reader.lines() {
        let string = line.expect("read line");
        let levels: Vec<i64> = string
            .trim()
            .split(' ')
            .flat_map(str::parse::<i64>)
            .collect();

        let report = { Report { levels: levels } };
        res.reports.push(report);
    }

    res
}

fn is_safe(report: &Report) -> bool {
    let mut differences: Vec<i64> = vec![];

    for i in 1..report.levels.len() {
        differences.push(report.levels[i] - report.levels[i - 1]);
    }

    differences.iter().all(|&e| e >= 1 && e <= 3) || differences.iter().all(|&e| e >= -3 && e <= -1)
}

fn is_safe_part2(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    for index_to_skip in 0..report.levels.len() {
        let mut new_levels: Vec<i64> = vec![];
        new_levels.extend(&report.levels[0..index_to_skip]);
        new_levels.extend(&report.levels[(index_to_skip + 1)..]);
        let new_report: Report = { Report { levels: new_levels } };
        if is_safe(&new_report) {
            return true;
        };
    }

    false
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let safe = input.reports.iter().filter(|&e| is_safe(e)).count();
    println!("{} safe reports", safe)
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let safe = input.reports.iter().filter(|&e| is_safe_part2(e)).count();
    println!("{} safe reports", safe)
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
