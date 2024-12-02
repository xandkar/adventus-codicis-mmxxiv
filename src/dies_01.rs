use std::{collections::HashMap, io::BufRead, iter};

use anyhow::{anyhow, bail};

pub struct Data {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> anyhow::Result<Self> {
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
            let fields: Vec<i32> = line
                .split_whitespace()
                .filter_map(|field| field.parse().ok())
                .collect();
            match &fields[..] {
                [n_left, n_right] => {
                    left.push(*n_left);
                    right.push(*n_right);
                }
                _ => {
                    bail!("bad input line: {line:?}. File={input:?}:{line_number}.");
                }
            }
        }
        Ok(Self { left, right })
    }

    pub fn solve1(&self) -> anyhow::Result<u32> {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        left.sort();
        right.sort();
        let total_distance: u32 =
            iter::zip(left, right).map(|(l, r)| l.abs_diff(r)).sum();
        Ok(total_distance)
    }

    pub fn solve2(&self) -> anyhow::Result<i32> {
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
            .map(|n| *n * *right_hist.get(n).unwrap_or(&0))
            .sum();
        Ok(similarity_score)
    }
}
