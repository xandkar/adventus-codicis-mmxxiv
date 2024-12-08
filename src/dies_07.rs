use std::{collections::HashSet, fs, str::FromStr};

use anyhow::{anyhow, Context};

pub struct Data {
    equations: Vec<Equation>,
}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = fs::read_to_string(path)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let mut equations = Vec::new();
        for (line_num, line) in
            input.lines().enumerate().map(|(i, l)| (i + 1, l))
        {
            let equation = line.parse().context(format!(
                "Invalid equation on line {line_num}: {line:?}"
            ))?;
            equations.push(equation);
        }
        Ok(Self { equations })
    }

    pub fn solve1(&self) -> anyhow::Result<i64> {
        let fs = &[Fun::Add, Fun::Mul];
        let sum = sum_of_possible_calibs(&self.equations[..], fs);
        Ok(sum)
    }

    pub fn solve2(&self) -> anyhow::Result<i64> {
        let fs = &[Fun::Add, Fun::Mul, Fun::Concat];
        let sum = sum_of_possible_calibs(&self.equations[..], fs);
        Ok(sum)
    }
}

fn sum_of_possible_calibs(eqs: &[Equation], fs: &[Fun]) -> i64 {
    eqs.iter()
        .filter(|eq| eq.could_be_true(fs))
        .map(|eq| eq.calib)
        .sum()
}

struct Equation {
    calib: i64,
    operands: Vec<i64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_whitespace();
        let calib_val =
            fields.next().ok_or(anyhow!("Missing calibration value"))?;
        let calib_val = calib_val
            .strip_suffix(":")
            .ok_or(anyhow!("Invalid calibration value"))?;
        let calib: i64 =
            calib_val.parse().context("Invalid calibration value")?;
        let mut operands: Vec<i64> = Vec::new();
        for field in fields {
            let operand: i64 = field.parse().context("Invalid operand")?;
            operands.push(operand);
        }
        Ok(Self { calib, operands })
    }
}

impl Equation {
    // TODO DP
    fn could_be_true(&self, fs: &[Fun]) -> bool {
        match &self.operands[..] {
            [x1, xs @ ..] if !xs.is_empty() => {
                let mut totals: HashSet<i64> = HashSet::from([*x1]);
                for &x in xs {
                    totals = totals
                        .into_iter()
                        .flat_map(|total| {
                            fs.iter().filter_map(move |f| f.apply(total, x))
                        })
                        .collect();
                }
                totals.contains(&self.calib)
            }
            _ => false,
        }
    }
}

enum Fun {
    Add,
    Mul,
    Concat,
}

impl Fun {
    fn apply(&self, left: i64, right: i64) -> Option<i64> {
        let output = match self {
            Self::Add => left + right,
            Self::Mul => left * right,
            Self::Concat => format!("{left}{right}").parse().ok()?,
        };
        Some(output)
    }
}
