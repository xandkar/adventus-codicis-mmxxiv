use std::{io::BufRead, iter::zip};

use anyhow::anyhow;

pub struct Data {
    reports: Vec<Vec<i32>>,
}

impl Data {
    pub fn load(input: &std::path::Path) -> anyhow::Result<Self> {
        let file = std::fs::File::open(input).map_err(|e| {
            anyhow!("Failure to open input file {:?}: {:?}", input, e)
        })?;
        let mut reports = Vec::new();
        for (line_num, line_result) in std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
        {
            let line = line_result?;
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|level| {
                    level
                        .parse()
                        .inspect_err(|e| {
                            eprintln!(
                                "Failed to parse level={level:?} \
                                on line={line_num} \
                                with error={e:?}"
                            );
                        })
                        .ok()
                })
                .collect();
            reports.push(report);
        }
        Ok(Self { reports })
    }

    pub fn solve1(&self) -> anyhow::Result<usize> {
        let mut safe_count: usize = 0;
        for report in &self.reports {
            if is_safe(&report[..]) {
                safe_count += 1;
            }
        }
        Ok(safe_count)
    }

    pub fn solve2(&self) -> anyhow::Result<usize> {
        let mut safe_count: usize = 0;
        'reports: for report in &self.reports {
            let n = report.len();
            for i in 0..=n {
                let is_safe = if i < 1 {
                    is_safe(&report[..])
                } else {
                    let i = i - 1;
                    let report_reduced: Vec<i32> = report
                        .iter()
                        .enumerate()
                        .filter_map(|(j, level)| (j != i).then_some(level))
                        .copied()
                        .collect();
                    is_safe(&report_reduced[..])
                };
                if is_safe {
                    safe_count += 1;
                    continue 'reports;
                }
            }
        }
        Ok(safe_count)
    }
}

fn is_safe(report: &[i32]) -> bool {
    let intervals: Vec<i32> = zip(&report[0..], &report[1..])
        .map(|(l, r)| r - l)
        .collect();
    let all_increasing =
        intervals.iter().filter(|i| **i > 0).count() == intervals.len();
    let all_decreasing =
        intervals.iter().filter(|i| **i < 0).count() == intervals.len();
    let all_in_range = intervals
        .iter()
        .filter(|i| (**i).abs() >= 1 && (**i).abs() <= 3)
        .count()
        == intervals.len();
    all_in_range && (all_increasing || all_decreasing)
}
