use std::fs;

pub struct Data {}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = fs::read_to_string(path)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        for (line_num, line) in
            input.lines().enumerate().map(|(i, l)| (i + 1, l))
        {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "";
        let data = Data::parse(input).unwrap();
        data.solve1().unwrap();
        data.solve2().unwrap();
    }
}
