use std::io::BufRead;

use anyhow::anyhow;

pub struct Data {}

impl Data {
    pub fn load(input: &std::path::Path) -> anyhow::Result<Self> {
        let file = std::fs::File::open(input).map_err(|e| {
            anyhow!("Failure to open input file {:?}: {:?}", input, e)
        })?;
        for (line_num, line_result) in std::io::BufReader::new(file)
            .lines()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
        {
            let line: String = line_result?;
            eprintln!("{line_num}: {line:?}");
        }
        Ok(Self {})
    }

    pub fn solve1(&self) -> anyhow::Result<u64> {
        todo!();
    }

    pub fn solve2(&self) -> anyhow::Result<u64> {
        todo!();
    }
}
