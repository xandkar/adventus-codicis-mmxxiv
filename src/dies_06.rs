use std::{
    collections::{HashMap, HashSet},
    fs,
};

use anyhow::anyhow;
use rayon::iter::{ParallelBridge, ParallelIterator};

pub struct Data {
    guard: Guard,
    grid: Grid,
}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = fs::read_to_string(path)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let grid: Vec<Vec<char>> =
            input.lines().map(|row| row.chars().collect()).collect();
        let mut guards: Vec<Guard> = grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(k, c)| Guard::from_char(*c, r, k))
            })
            .collect();
        let guard = guards.pop().ok_or(anyhow!("No guard found!"))?;
        guards
            .is_empty()
            .then_some(())
            .ok_or(anyhow!("Too many guards: {:?}", (&guard, guards)))?;
        let grid: Vec<Vec<bool>> = grid
            .iter()
            .map(|row| row.iter().map(|c| *c == '#').collect())
            .collect();
        let row_lengths: HashSet<usize> =
            grid.iter().map(|row| row.len()).collect();
        assert_eq!(1, row_lengths.len(), "Rectangle.");
        assert_eq!(grid.len(), grid[0].len(), "Square.");
        Ok(Self { grid, guard })
    }

    pub fn solve1(&self) -> anyhow::Result<usize> {
        let mut guard = self.guard.clone();
        guard.patrol(&self.grid);
        let unique_positions_visited: HashSet<(usize, usize)> =
            guard.visits.iter().map(|((pos, _), _)| *pos).collect();
        Ok(unique_positions_visited.len())
    }

    // XXX Terrible brute-force solution.
    // TODO Look into https://en.wikipedia.org/wiki/Cycle_detection
    pub fn solve2(&self) -> anyhow::Result<i32> {
        let positions: HashSet<(usize, usize)> = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter().enumerate().filter_map(
                    move |(k, is_obstructed)| {
                        (!is_obstructed).then_some((r, k))
                    },
                )
            })
            .collect();
        let count = positions
            .iter()
            .par_bridge()
            .map({
                |(r, k)| {
                    let mut grid = self.grid.clone();
                    grid[*r][*k] = true;
                    let mut guard = self.guard.clone();
                    match guard.patrol(&grid) {
                        Outcome::Exited => 0,
                        Outcome::Looped => 1,
                    }
                }
            })
            .sum();
        Ok(count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn turn(&self) -> Self {
        match self {
            Self::N => Self::E,
            Self::E => Self::S,
            Self::S => Self::W,
            Self::W => Self::N,
        }
    }
}

type Grid = Vec<Vec<bool>>;

enum View {
    Exit,
    Obstructed,
    Free(usize, usize),
}

enum Outcome {
    Exited,
    Looped,
}

#[derive(Debug, Clone)]
struct Guard {
    dir: Dir,
    pos: (usize, usize),
    visits: HashMap<((usize, usize), Dir), usize>,
    total_moves: usize,
}

impl Guard {
    fn turn(&mut self) {
        self.dir = self.dir.turn();
    }

    fn move_to(&mut self, r: usize, k: usize) {
        let pos = (r, k);
        self.visits
            .entry((pos, self.dir))
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.total_moves += 1;
        self.pos = pos;
    }

    fn look(&self, grid: &[Vec<bool>]) -> View {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let r0 = self.pos.0 as i32;
        let k0 = self.pos.1 as i32;
        let (r1, k1) = match self.dir {
            Dir::N => (r0 - 1, k0),
            Dir::E => (r0, k0 + 1),
            Dir::S => (r0 + 1, k0),
            Dir::W => (r0, k0 - 1),
        };
        let is_in_bounds = r1 >= 0 && r1 < rows && k1 >= 0 && k1 < cols;
        let r1 = r1 as usize;
        let k1 = k1 as usize;
        if is_in_bounds {
            if grid[r1][k1] {
                View::Obstructed
            } else {
                View::Free(r1, k1)
            }
        } else {
            View::Exit
        }
    }

    fn patrol(&mut self, grid: &[Vec<bool>]) -> Outcome {
        loop {
            match self.look(grid) {
                View::Exit => {
                    return Outcome::Exited;
                }
                View::Obstructed => {
                    self.turn();
                }
                View::Free(r1, k1) => {
                    // XXX Experimentally-found limit:
                    if self.total_moves > 6_000 {
                        return Outcome::Looped;
                    }
                    if let Some(count) =
                        self.visits.get(&((r1, k1), self.dir))
                    {
                        // XXX Somewhat-arbitrary heuristic:
                        if *count > 2 {
                            return Outcome::Looped;
                        }
                    }
                    self.move_to(r1, k1);
                }
            }
        }
    }
}

impl Guard {
    fn from_char(c: char, r: usize, k: usize) -> Option<Self> {
        let dir = match c.to_ascii_uppercase() {
            '^' => Dir::N,
            '>' => Dir::E,
            'V' => Dir::S,
            '<' => Dir::W,
            _ => return None,
        };
        let pos = (r, k);
        let visited = HashMap::from([((pos, dir), 1)]);
        let selph = Self {
            dir,
            pos,
            visits: visited,
            total_moves: 0,
        };
        Some(selph)
    }
}
