use aoc_2024::grid::Grid;

#[derive(Debug)]
struct Input {
    grid: Grid<u8>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    Input {
        grid: Grid::from_reader(reader, |line| line.into_bytes()),
    }
}

fn find_straight(grid: &Grid<u8>, search: &[u8], pos: (usize, usize)) -> usize {
    grid.directions()
        .into_iter()
        .filter(|&direction| grid.line(pos, direction, search.len()) == search)
        .count()
}

fn find_cross(grid: &Grid<u8>, pos: (usize, usize)) -> usize {
    if pos.0 == 0 || pos.0 == grid.width() - 1 || pos.1 == 0 || pos.1 == grid.height() - 1 {
        // If we're on the edge, we can't be the middle of an X-MAS
        return 0;
    }
    if grid.at(pos) != 65 {
        // If it's not an A then this spot is not the middle of an X-MAS
        return 0;
    }

    let diag1 = vec![(pos.0 - 1, pos.1 - 1), pos, (pos.0 + 1, pos.1 + 1)];
    let diag2 = vec![(pos.0 - 1, pos.1 + 1), pos, (pos.0 + 1, pos.1 - 1)];
    let cross1 = vec![(pos.0, pos.1 - 1), pos, (pos.0, pos.1 + 1)];
    let cross2 = vec![(pos.0 - 1, pos.1), pos, (pos.0 + 1, pos.1)];

    let mas = "MAS".as_bytes();
    let sam = "SAM".as_bytes();

    let d1: Vec<u8> = diag1.iter().map(|&p| grid.at(p)).collect();
    let d2: Vec<u8> = diag2.iter().map(|&p| grid.at(p)).collect();

    if (d1 == mas || d1 == sam) && (d2 == mas || d2 == sam) {
        println!("   found diagonal");
        return 1;
    }

    /*
     * Part 2 does not allow + arrangement, just X

    let c1: Vec<u8> = cross1.iter().map(|&p| grid.at(p)).collect();
    let c2: Vec<u8> = cross2.iter().map(|&p| grid.at(p)).collect();

    if (c1 == mas || c1 == sam) && (c2 == mas || c2 == sam) {
        println!("   found vert/hor");
        return 1;
    }
    */

    0
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let search = "XMAS".as_bytes();
    let mut count = 0;

    for x in 0..input.grid.width() {
        for y in 0..input.grid.height() {
            count += find_straight(&input.grid, search, (x, y));
        }
    }

    println!("found {}", count);
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let mut count = 0;

    for x in 0..input.grid.width() {
        for y in 0..input.grid.height() {
            count += find_cross(&input.grid, (x, y));
        }
    }

    println!("found {}", count);
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
