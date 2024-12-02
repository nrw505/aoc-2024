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

    let mut i = 1;
    while i < report.levels.len() {
        differences.push(report.levels[i] - report.levels[i - 1]);
        i += 1;
    }

    differences.iter().all(|&e| e >= 1 && e <= 3) || differences.iter().all(|&e| e >= -3 && e <= -1)
}

fn is_safe_part2(report: &Report) -> bool {
    if is_safe(report) {
        return true;
    }

    let mut index_to_skip = 0;
    while index_to_skip < report.levels.len() {
        let new_report: Report = {
            Report {
                levels: report
                    .levels
                    .iter()
                    .enumerate()
                    .flat_map(|(index, &value)| {
                        if index == index_to_skip {
                            None
                        } else {
                            Some(value)
                        }
                    })
                    .collect(),
            }
        };
        if is_safe(&new_report) {
            return true;
        };
        index_to_skip += 1;
    }

    false
}

fn day2_part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut safe: i64 = 0;
    for report in input.reports {
        if is_safe(&report) {
            safe += 1;
        }
    }
    println!("{} safe reports", safe)
}

fn day2_part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut safe: i64 = 0;
    for report in input.reports {
        if is_safe_part2(&report) {
            safe += 1;
        }
    }
    println!("{} safe reports", safe)
}

pub fn day2(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(day2_part1(reader)),
        2 => Ok(day2_part2(reader)),
        _ => todo!(),
    }
}
