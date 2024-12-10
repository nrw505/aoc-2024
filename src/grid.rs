#[derive(Debug)]
pub struct Grid<T: Copy + PartialEq + Eq + std::hash::Hash> {
    rows: Vec<Vec<T>>,
}

impl<T: Copy + PartialEq + Eq + std::hash::Hash> Grid<T> {
    pub fn from_reader(reader: impl std::io::BufRead, splitter: fn(String) -> Vec<T>) -> Grid<T> {
        let mut res = Grid::<T> { rows: vec![] };

        for line in reader.lines() {
            res.rows.push(splitter(line.expect("line read")))
        }

        res
    }

    pub fn at(&self, pos: (usize, usize)) -> T {
        let (x, y) = pos;

        self.rows[y][x]
    }

    pub fn width(&self) -> usize {
        // Assuming all rows are the same width for now (was true for day 4...)
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn directions(&self) -> Vec<(isize, isize)> {
        vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ]
    }

    pub fn directions_without_diagonals(&self) -> Vec<(isize, isize)> {
        vec![(0, -1), (1, 0), (0, 1), (-1, 0)]
    }

    pub fn reverse(&self, dir: (isize, isize)) -> (isize, isize) {
        (-dir.0, -dir.1)
    }

    pub fn add_dir_to_pos(&self, pos: (usize, usize), dir: (isize, isize)) -> (usize, usize) {
        (
            (pos.0 as isize + dir.0) as usize,
            (pos.1 as isize + dir.1) as usize,
        )
    }

    pub fn neighbours(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;

        let mut res: Vec<(usize, usize)> = vec![];
        let mut xs: Vec<usize> = vec![x];
        let mut ys: Vec<usize> = vec![y];

        if x > 0 {
            xs.push(x - 1)
        }
        if x < (self.width() as usize - 1) {
            xs.push(x + 1)
        }
        if y > 0 {
            ys.push(y - 1)
        }
        if y < (self.height() as usize - 1) {
            ys.push(y + 1)
        }

        for &nx in xs.iter() {
            for &ny in ys.iter() {
                if (nx, ny) != (x, y) {
                    res.push((nx, ny))
                }
            }
        }

        res
    }

    pub fn line(&self, pos: (usize, usize), dir: (isize, isize), len: usize) -> Vec<T> {
        let mut res = vec![];

        let w = self.width() as isize;
        let h = self.height() as isize;

        for i in 0..len {
            let x = pos.0 as isize + i as isize * dir.0;
            let y = pos.1 as isize + i as isize * dir.1;

            if x >= 0 && x < w && y >= 0 && y < h {
                res.push(self.at((x as usize, y as usize)))
            }
        }
        res
    }

    pub fn find(&self, item: T) -> Option<(usize, usize)> {
        let w = self.width();
        let h = self.height();

        for x in 0..w {
            for y in 0..h {
                if self.at((x, y)) == item {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn find_all(&self, item: T) -> Vec<(usize, usize)> {
        let w = self.width();
        let h = self.height();

        let mut res = vec![];

        for x in 0..w {
            for y in 0..h {
                if self.at((x, y)) == item {
                    res.push((x, y));
                }
            }
        }
        res
    }

    pub fn position_in_bounds(&self, pos: (usize, usize)) -> bool {
        pos.0 < self.width() && pos.1 < self.height()
    }

    pub fn distance(&self, pos1: (usize, usize), pos2: (usize, usize)) -> (isize, isize) {
        (
            pos2.0 as isize - pos1.0 as isize,
            pos2.1 as isize - pos1.1 as isize,
        )
    }

    pub fn labels(&self) -> std::collections::HashSet<T> {
        let mut ret = std::collections::HashSet::new();

        let w = self.width();
        let h = self.height();

        for x in 0..(w - 1) {
            for y in 0..(h - 1) {
                ret.insert(self.at((x, y)));
            }
        }

        ret
    }
}
