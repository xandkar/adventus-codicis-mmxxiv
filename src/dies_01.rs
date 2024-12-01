use std::{collections::HashMap, io::BufRead, ops::Sub};

use anyhow::{anyhow, Context, Result};

pub struct Data {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> Result<Self> {
        let file = std::fs::File::open(input).map_err(|e| {
            anyhow!("Failure to open input file {:?}: {:?}", input, e)
        })?;
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (line_number, line_result) in std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
        {
            let line = line_result?;
            // Closure because no .clone()
            let err = || {
                anyhow!("bad input line: {line:?}. File={input:?}. Line number={line_number}.")
            };
            let mut fields = line.split_whitespace();
            let n_left: &str = fields.next().ok_or(err())?;
            let n_right: &str = fields.next().ok_or(err())?;
            let n_left: i32 = n_left.parse().context(err())?;
            let n_right: i32 = n_right.parse().context(err())?;
            left.push(n_left);
            right.push(n_right);
        }
        Ok(Self { left, right })
    }

    pub fn solve1(&self) -> Result<i32> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        left.sort();
        right.sort();
        let total_distance: i32 = left
            .into_iter()
            .zip(right.into_iter())
            .map(|(l, r)| l.sub(r).abs())
            .sum();
        Ok(total_distance)
    }

    pub fn solve2(&self) -> Result<i32> {
        let mut right_hist = HashMap::new();
        for n in &self.right {
            right_hist
                .entry(*n)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let similarity_score: i32 = self
            .left
            .iter()
            .map(|n| {
                let m = right_hist.get(n).unwrap_or(&0);
                *n * *m
            })
            .sum();
        Ok(similarity_score)
    }
}
