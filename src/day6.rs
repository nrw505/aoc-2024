use aoc_2024::grid::Grid;

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

fn rotate_right(dir: (isize, isize)) -> (isize, isize) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => dir,
    }
}

fn part1(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let height = input.grid.height();
    let width = input.grid.width();

    let mut pos = input.grid.find('^').unwrap();
    let mut dir: (isize, isize) = (0, -1);

    let mut positions: HashSet<(usize, usize)> = HashSet::new();

    while pos.0 < width && pos.1 < height {
        positions.insert(pos);

        let newpos = (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        );

        if input.grid.position_in_bounds(newpos) && input.grid.at(newpos) == '#' {
            dir = rotate_right(dir);
        } else {
            pos = newpos;
        }

        // println!("at {:?}, pointing {:?}, visited: {:?}", pos, dir, positions);
    }

    println!("visited {:?}", positions.len());
}

fn causes_a_loop(
    grid: &Grid<char>,
    start_pos: (usize, usize),
    start_dir: (isize, isize),
    obstacle: (usize, usize),
) -> bool {
    // println!("loop check start = {:?}, obstacle = {:?}", start, obstacle);
    let mut pos = start_pos;
    let mut dir = start_dir;
    let height = grid.height();
    let width = grid.width();

    let mut positions: HashSet<((usize, usize), (isize, isize))> = HashSet::new();

    while pos.0 < width && pos.1 < height {
        positions.insert((pos, dir));

        let newpos = grid.add_dir_to_pos(pos, dir);
        let r_right = rotate_right(dir);

        if !grid.position_in_bounds(newpos) {
            return false;
        }

        if newpos == obstacle || grid.at(newpos) == '#' {
            dir = r_right;
        } else {
            pos = newpos;
        }

        if positions.contains(&(pos, dir)) {
            return true;
        }
    }

    false
}

fn part2(reader: impl std::io::BufRead) {
    let input = parse_input(reader);

    let height = input.grid.height();
    let width = input.grid.width();

    let startpos = input.grid.find('^').unwrap();
    let startdir: (isize, isize) = (0, -1);

    let mut pos = startpos;
    let mut dir = startdir;

    let mut positions: HashSet<(usize, usize)> = HashSet::new();
    let mut new_obstacles: HashSet<(usize, usize)> = HashSet::new();

    while pos.0 < width && pos.1 < height {
        positions.insert(pos);

        let newpos = input.grid.add_dir_to_pos(pos, dir);
        let r_right = rotate_right(dir);

        if input.grid.position_in_bounds(newpos) && input.grid.at(newpos) == '#' {
            dir = r_right;
        } else {
            pos = newpos;
        }
    }

    let pos_count = positions.len();
    println!("{:?} positions", pos_count);

    let mut i = 0;
    for location in positions {
        println!("checking option {} of {}", i, pos_count);

        if location == startpos {
            println!("skipping guard start space");
            continue;
        }

        if causes_a_loop(&input.grid, startpos, startdir, location) {
            new_obstacles.insert(location);
        }
        i = i + 1;
    }

    println!("{:?} possible loop causers", new_obstacles.len());
}

pub fn doit(part: u32, reader: impl std::io::BufRead) -> std::io::Result<()> {
    match part {
        1 => Ok(part1(reader)),
        2 => Ok(part2(reader)),
        _ => todo!(),
    }
}
