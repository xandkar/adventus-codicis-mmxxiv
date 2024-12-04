use std::collections::HashSet;

pub struct Data {
    grid: Vec<Vec<char>>,
}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = std::fs::read_to_string(path)?;
        Self::parse(&input)
    }

    fn parse(input: &str) -> anyhow::Result<Self> {
        let mut grid = Vec::new();
        for line in input.lines() {
            let row: Vec<char> = line.to_uppercase().chars().collect();
            grid.push(row);
        }
        let row_lengths: HashSet<usize> =
            grid.iter().map(|row| row.len()).collect();
        assert_eq!(1, row_lengths.len(), "Rectangle.");
        assert_eq!(grid.len(), grid[0].len(), "Square.");
        Ok(Self { grid })
    }

    pub fn solve1(&self) -> anyhow::Result<usize> {
        let target = "XMAS".to_uppercase();
        let target_rev: String = target.chars().rev().collect();
        let mut target_count = 0;
        let n_rows = self.grid.len();
        let n_cols = self.grid[0].len();
        let mut maybe_count = |word: &str| {
            if word == target || word == target_rev {
                target_count += 1;
            }
        };

        // Rows:
        for r in 0..n_rows {
            for k in 0..=(n_cols - target.len()) {
                let word = self.grid[r][k..k + target.len()]
                    .iter()
                    .collect::<String>();
                maybe_count(&word);
            }
        }

        // Columns:
        for k in 0..n_cols {
            for r in 0..=(n_rows - target.len()) {
                let mut word = String::new();
                for i in 0..target.len() {
                    word.push(self.grid[r + i][k])
                }
                maybe_count(&word);
            }
        }

        // Across 1
        for k in 0..n_cols {
            let k = k as i32;
            for r in 0..=(n_rows - target.len()) {
                let r = r as i32;
                let mut word = String::new();
                for i in 0..target.len() {
                    let i = i as i32;
                    let r = r + i;
                    let k = k - i;
                    if r >= 0
                        && r < n_rows as i32
                        && k >= 0
                        && k < n_cols as i32
                    {
                        word.push(self.grid[r as usize][k as usize])
                    }
                }
                maybe_count(&word);
            }
        }

        // Across 2
        for k in (0..n_cols).rev() {
            let k = k as i32;
            for r in 0..=(n_rows - target.len()) {
                let r = r as i32;
                let mut word = String::new();
                for i in 0..target.len() {
                    let i = i as i32;
                    let r = r + i;
                    let k = k + i;
                    if r >= 0
                        && r < n_rows as i32
                        && k >= 0
                        && k < n_cols as i32
                    {
                        word.push(self.grid[r as usize][k as usize])
                    }
                }
                maybe_count(&word);
            }
        }

        Ok(target_count)
    }

    pub fn solve2(&self) -> anyhow::Result<usize> {
        let n_rows = self.grid.len();
        let n_cols = self.grid[0].len();
        let mut count = 0;
        for r in 0..n_rows {
            for k in 0..n_cols {
                let inner = self.grid[r][k];
                if inner == 'A' {
                    let outter: Vec<char> = [
                        Loc { r: -1, k: -1 }, // NW
                        Loc { r: -1, k: 1 },  // NE
                        Loc { r: 1, k: 1 },   // SE
                        Loc { r: 1, k: -1 },  // SW
                    ]
                    .iter()
                    .map(|o| Loc {
                        r: (r as i32) + o.r,
                        k: (k as i32) + o.k,
                    })
                    .filter(|loc| is_inbounds(n_cols, n_rows, loc))
                    .map(|loc| &self.grid[loc.r as usize][loc.k as usize])
                    .copied()
                    .filter(|c| "MS".contains(*c))
                    .collect();
                    if outter.len() == 4
                        && outter[0] != outter[2]
                        && outter[1] != outter[3]
                    {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }
}

fn is_inbounds(cols: usize, rows: usize, l: &Loc) -> bool {
    l.r >= 0 && l.r < rows as i32 && l.k >= 0 && l.k < cols as i32
}

#[derive(Debug)]
struct Loc {
    r: i32,
    k: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_example() {
        let input_a = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let input_b = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, Data::parse(input_a).unwrap().solve2().unwrap());
        assert_eq!(9, Data::parse(input_b).unwrap().solve2().unwrap());
        ();
    }
}
