use aoc_2024::grid::Grid;

use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    grid: Grid<char>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    Input {
        grid: Grid::from_reader(reader, |line| line.chars().collect()),
    }
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let freqs: Vec<char> = input
        .grid
        .labels()
        .into_iter()
        .filter(|&e| e != '.')
        .collect();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for freq in freqs {
        let locations = input.grid.find_all(freq);

        let pairs: Vec<Vec<(usize, usize)>> = locations.into_iter().combinations(2).collect();

        for pair in pairs {
            let d1 = input.grid.distance(pair[0], pair[1]);
            let d2 = input.grid.distance(pair[1], pair[0]);

            let an1 = input.grid.add_dir_to_pos(pair[1], d1);
            let an2 = input.grid.add_dir_to_pos(pair[0], d2);

            if input.grid.position_in_bounds(an1) {
                antinodes.insert(an1);
            }
            if input.grid.position_in_bounds(an2) {
                antinodes.insert(an2);
            }
        }
    }

    println!("{} distinct antinode locations", antinodes.len());
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let freqs: Vec<char> = input
        .grid
        .labels()
        .into_iter()
        .filter(|&e| e != '.')
        .collect();

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for freq in freqs {
        let locations = input.grid.find_all(freq);

        let pairs: Vec<Vec<(usize, usize)>> = locations.into_iter().combinations(2).collect();

        for pair in pairs {
            let mut d = input.grid.distance(pair[0], pair[1]);

            let mut pos = pair[1];
            while input.grid.position_in_bounds(pos) {
                antinodes.insert(pos);
                pos = input.grid.add_dir_to_pos(pos, d);
            }

            pos = pair[0];
            d = input.grid.reverse(d);
            while input.grid.position_in_bounds(pos) {
                antinodes.insert(pos);
                pos = input.grid.add_dir_to_pos(pos, d);
            }
        }
    }

    println!("{} distinct antinode locations", antinodes.len());
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
