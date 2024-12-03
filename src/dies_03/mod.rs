mod parser;

#[derive(Debug, PartialEq)]
pub enum Ix {
    Do,
    Dont,
    Mul(i32, i32),
}

pub struct Data {
    ixs: Vec<Ix>,
}

impl From<Vec<Ix>> for Data {
    fn from(ixs: Vec<Ix>) -> Self {
        Self { ixs }
    }
}

impl Data {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        let input = std::fs::read_to_string(path)?;
        Self::parse(&input)
    }

    pub fn parse(input: &str) -> anyhow::Result<Self> {
        let ixs = parser::parse(input);
        Ok(Self::from(ixs))
    }

    pub fn solve1(&self) -> anyhow::Result<i32> {
        let mut sum_of_products = 0;
        for ix in self.ixs.iter() {
            match ix {
                Ix::Do | Ix::Dont => (),
                Ix::Mul(left, right) => {
                    sum_of_products += *left * *right;
                }
            }
        }
        Ok(sum_of_products)
    }

    pub fn solve2(&self) -> anyhow::Result<i32> {
        let mut sum_of_products = 0;
        let mut enabled = true;
        for ix in self.ixs.iter() {
            match ix {
                Ix::Do => enabled = true,
                Ix::Dont => enabled = false,
                Ix::Mul(left, right) if enabled => {
                    sum_of_products += *left * *right;
                }
                Ix::Mul(_, _) => {}
            }
        }
        Ok(sum_of_products)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let data = Data::parse(input).unwrap();
        assert_eq!(161, data.solve1().unwrap());
    }

    #[test]
    fn example_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let data = Data::parse(input).unwrap();
        assert_eq!(48, data.solve2().unwrap());
    }

    #[test]
    fn interp_part_1() {
        let data = Data::from(vec![
            Ix::Mul(2, 4),
            Ix::Mul(5, 5),
            Ix::Mul(11, 8),
            Ix::Mul(8, 5),
        ]);
        assert_eq!(161, data.solve2().unwrap())
    }

    #[test]
    fn interp_part_2() {
        let data = Data::from(vec![
            Ix::Mul(2, 4),
            Ix::Dont,
            Ix::Mul(5, 5),
            Ix::Mul(11, 8),
            Ix::Do,
            Ix::Mul(8, 5),
        ]);
        assert_eq!(48, data.solve2().unwrap())
    }
}
