use std::{iter::Peekable, str::Chars};

use super::Ix;

pub fn parse(input: &str) -> Vec<Ix> {
    let mut ixs = Vec::new();
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        let ix_opt = parse_ix(&mut chars);
        if let Some(ix) = ix_opt {
            ixs.push(ix);
        }
    }
    ixs
}

fn parse_ix(chars: &mut Peekable<Chars>) -> Option<Ix> {
    consume_non_key(chars)?;
    let key_candidate = parse_key_candidate(chars)?;
    // Checking the ends, rather than wholes, makes it easier to
    // ignore noise prefixes:
    if key_candidate.ends_with("do") {
        consume_unit(chars)?;
        Some(Ix::Do)
    } else if key_candidate.ends_with("don't") {
        consume_unit(chars)?;
        Some(Ix::Dont)
    } else if key_candidate.ends_with("mul") {
        consume_paren_open(chars)?;
        let left = parse_num(chars)?;
        consume_comma(chars)?;
        let right = parse_num(chars)?;
        consume_paren_close(chars)?;
        Some(Ix::Mul(left, right))
    } else {
        None
    }
}
fn parse_num(chars: &mut Peekable<Chars>) -> Option<i32> {
    let mut num = None;
    let mut buf = String::new();
    while let Some(c) = chars.peek() {
        if c.is_ascii_digit() {
            buf.push(*c);
            chars.next().unwrap_or_else(|| unreachable!());
        } else {
            num = Some(buf.parse().unwrap_or_else(|_| unreachable!()));
            break;
        }
    }
    num
}

fn is_key_char(c: char) -> bool {
    "don'tmul".contains(c)
}

fn parse_key_candidate(chars: &mut Peekable<Chars>) -> Option<String> {
    let mut key = None;
    let mut buf = String::new();
    while let Some(c) = chars.peek() {
        if is_key_char(*c) {
            buf.push(*c);
            chars.next().unwrap_or_else(|| unreachable!());
        } else {
            key = Some(buf.to_string());
            break;
        }
    }
    key
}

fn consume_non_key(chars: &mut Peekable<Chars>) -> Option<()> {
    while let Some(c) = chars.peek() {
        if is_key_char(*c) {
            return Some(());
        } else {
            chars.next().unwrap_or_else(|| unreachable!());
        }
    }
    None
}

fn consume_unit(chars: &mut Peekable<Chars>) -> Option<()> {
    consume_paren_open(chars)?;
    consume_paren_close(chars)?;
    Some(())
}

fn consume_paren_open(chars: &mut Peekable<Chars>) -> Option<()> {
    consume_char(chars, '(')
}

fn consume_paren_close(chars: &mut Peekable<Chars>) -> Option<()> {
    consume_char(chars, ')')
}

fn consume_comma(chars: &mut Peekable<Chars>) -> Option<()> {
    consume_char(chars, ',')
}

fn consume_char(chars: &mut Peekable<Chars>, c: char) -> Option<()> {
    if c == *chars.peek()? {
        chars.next().unwrap_or_else(|| unreachable!());
        Some(())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected =
            vec![Ix::Mul(2, 4), Ix::Mul(5, 5), Ix::Mul(11, 8), Ix::Mul(8, 5)];
        assert_eq!(expected, parse_with_regex(input).unwrap());
        assert_eq!(expected, parse(input));
    }

    #[test]
    fn part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected = vec![
            Ix::Mul(2, 4),
            Ix::Dont,
            Ix::Mul(5, 5),
            Ix::Mul(11, 8),
            Ix::Do,
            Ix::Mul(8, 5),
        ];
        assert_eq!(expected, parse_with_regex(input).unwrap());
        assert_eq!(expected, parse(input));
    }

    fn parse_with_regex(input: &str) -> anyhow::Result<Vec<Ix>> {
        let re = r#"(?x)
        (?P<do>do\(\))              | # Match "do()"
        (?P<dont>don't\(\))         | # Match "don't()"
        (?P<mul>mul\((\d+),(\d+)\))   # Match "mul(d1,d2)"
        "#;
        let re = regex::Regex::new(re)?;
        let mut ixs: Vec<Ix> = Vec::new();
        for cap in re.captures_iter(input) {
            let ix = if cap.name("do").is_some() {
                Ix::Do
            } else if cap.name("dont").is_some() {
                Ix::Dont
            } else if cap.name("mul").is_some() {
                let left: i32 = cap.get(4).unwrap().as_str().parse().unwrap();
                let right: i32 =
                    cap.get(5).unwrap().as_str().parse().unwrap();
                Ix::Mul(left, right)
            } else {
                unreachable!();
            };
            ixs.push(ix);
        }
        Ok(ixs)
    }
}
