use aoc_2024::grid::Grid;
use std::collections::HashSet;

#[derive(Debug)]
struct Input {
    grid: Grid<u8>,
}

fn parse_input(reader: impl std::io::BufRead) -> Input {
    Input {
        grid: Grid::from_reader(reader, |line| {
            line.chars()
                .map(|c| (c as u32 - '0' as u32) as u8)
                .collect()
        }),
    }
}

fn paths_up(grid: &Grid<u8>, from: (usize, usize)) -> u32 {
    let current_height = grid.at(from);

    if current_height == 9 {
        return 1;
    }

    grid.directions_without_diagonals()
        .iter()
        .map(|&dir| grid.add_dir_to_pos(from, dir))
        .filter(|&pos| grid.position_in_bounds(pos))
        .filter(|&pos| grid.at(pos) == current_height + 1)
        .map(|pos| paths_up(grid, pos))
        .sum()
}

fn peaks_reachable(grid: &Grid<u8>, from: (usize, usize)) -> HashSet<(usize, usize)> {
    let current_height = grid.at(from);

    if current_height == 9 {
        return HashSet::from([from]);
    }

    grid.directions_without_diagonals()
        .iter()
        .map(|&dir| grid.add_dir_to_pos(from, dir))
        .filter(|&pos| grid.position_in_bounds(pos))
        .filter(|&pos| grid.at(pos) == current_height + 1)
        .flat_map(|pos| peaks_reachable(grid, pos))
        .collect()
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let starts = input.grid.find_all(0);

    let peak_count = starts
        .iter()
        .map(|&start| peaks_reachable(&input.grid, start).len())
        .sum::<usize>();
    println!("{} peaks from {} starts", peak_count, starts.len());
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let starts = input.grid.find_all(0);

    let path_count = starts
        .iter()
        .map(|&start| paths_up(&input.grid, start))
        .sum::<u32>();

    println!("{} paths from {} starts", path_count, starts.len());
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
