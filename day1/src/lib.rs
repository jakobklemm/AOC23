use std::{fs, usize};

use anyhow::{anyhow, Result};

static DIGITS: &'static [&'static str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_line(line: &str) -> Result<usize> {
    let mut digits = Vec::new();
    for s in line.chars() {
        if let Ok(i) = s.to_string().parse::<usize>() {
            digits.push(i);
        }
    }
    let first = digits.first();
    let last = digits.last();

    if !first.is_none() && !last.is_none() {
        Ok(first.unwrap() * 10 + last.unwrap())
    } else {
        Err(anyhow!("Missing numbers in line"))
    }
}

fn parse_with_digits(line: &str) -> Result<usize> {
    Ok(parse_first(line)? * 10 + parse_last(line)?)
}

fn parse_first(line: &str) -> Result<usize> {
    let (d_i, d_v) = index_first_digit(line).unwrap_or((usize::MAX, 0));
    let (l_i, l_v) = index_first_letter(line).unwrap_or((usize::MAX, 0));

    // first found index is letter
    if d_i > l_i {
        Ok(l_v)
    } else if d_i < l_i {
        Ok(d_v)
    } else {
        Err(anyhow!("No valid digit found in forward pass"))
    }
}

fn parse_last(line: &str) -> Result<usize> {
    let (d_i, d_v) = index_last_digit(line).unwrap_or((i32::MIN, 0));
    let (l_i, l_v) = index_last_letter(line).unwrap_or((i32::MIN, 0));

    // first found index is letter
    if d_i < l_i {
        Ok(l_v)
    } else if d_i > l_i {
        Ok(d_v)
    } else {
        println!("Line: {}", line);
        println!("Digit: {} - {}", d_i, d_v);
        println!("Lettr: {} - {}", l_i, l_v);

        Err(anyhow!("No valid digit found in backwards pass"))
    }
}

fn index_first_letter(line: &str) -> Option<(usize, usize)> {
    for (i, _c) in line.chars().enumerate() {
        for term in DIGITS {
            if search_forwarad(line, i, term) {
                return Some((i, convert_digit(term)?));
            }
        }
    }
    None
}

fn index_last_letter(line: &str) -> Option<(i32, usize)> {
    let rev = line.chars().rev().collect::<String>();
    for (i, _c) in line.chars().enumerate() {
        for term in DIGITS {
            let revs = term.chars().rev().collect::<String>();
            if search_forwarad(&rev, i, &revs) {
                return Some((
                    ((line.len() - i) - term.len()).try_into().unwrap(),
                    convert_digit(term)?,
                ));
            }
        }
    }

    None
}

fn convert_digit(digit: &str) -> Option<usize> {
    let d = match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => return None,
    };
    return Some(d);
}

fn index_first_digit(line: &str) -> Option<(usize, usize)> {
    for (i, c) in line.chars().enumerate() {
        if let Ok(num) = c.to_string().parse::<usize>() {
            return Some((i, num));
        }
    }

    None
}

fn index_last_digit(line: &str) -> Option<(i32, usize)> {
    let s = line.chars().rev().collect::<String>();
    let (i, v) = index_first_digit(&s)?;
    Some(((line.len() - (i + 1)).try_into().unwrap(), v))
}

fn search_forwarad(line: &str, start: usize, term: &str) -> bool {
    if line.len() < term.len() || start + term.len() > line.len() {
        return false;
    }
    let iter = line.chars().skip(start);
    let mut index = 0;
    for s in iter {
        if let Some(chr) = term.chars().nth(index) {
            // println!("Index: {}, Source: {}, Search: {}", index, s, chr);
            if chr != s {
                return false;
            } else {
                if index + 1 == term.len() {
                    return true;
                }
            }
        } else {
            return false;
        }
        index += 1;
    }

    true
}

pub fn driver() -> Result<usize> {
    let fs = fs::read_to_string("input.txt")?;
    let mut sum = 0;
    for l in fs.lines() {
        sum += parse_line(l)?;
    }
    Ok(sum)
}

pub fn test_data() -> Result<usize> {
    let fs = fs::read_to_string("test.txt")?;
    let mut sum = 0;
    for l in fs.lines() {
        sum += parse_with_digits(l)?;
    }
    Ok(sum)
}

pub fn driver_complete() -> Result<usize> {
    let fs = fs::read_to_string("input.txt")?;
    let mut sum = 0;
    for l in fs.lines() {
        sum += parse_with_digits(l)?;
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete() {
        assert_eq!(driver_complete().unwrap(), 0);
    }

    #[test]
    fn test_debug() {
        assert_eq!(test_data().unwrap(), 281);
    }

    #[test]
    fn test_parse_last() {
        assert_eq!(parse_last("kkone2ee").unwrap(), 2);
    }

    #[test]
    fn test_index_last_letter() {
        assert_eq!(index_last_letter("ttttone").unwrap().1, 1);
        assert_eq!(index_last_letter("ttttonetwothree4").unwrap().1, 3);
        assert_eq!(index_last_letter("aone").unwrap(), (1, 1));
        assert_eq!(index_last_letter("kkone2ee").unwrap(), (2, 1));
    }

    #[test]
    fn test_index_last_digit() {
        assert_eq!(index_last_digit("abcone1tht").unwrap().1, 1);
        assert_eq!(index_last_digit("8abcone1tht").unwrap().1, 1);
        assert_eq!(index_last_digit("kkone2ee").unwrap(), (5, 2));
    }

    #[test]
    fn test_search_forward() {
        assert_eq!(search_forwarad("aone", 0, "one"), false);
        assert_eq!(search_forwarad("aone", 1, "one"), true);
        assert_eq!(search_forwarad("a", 0, "one"), false);
        assert_eq!(search_forwarad("aono", 3, "one"), false);
    }

    #[test]
    fn test_find_first() {
        assert_eq!(parse_first("aaone2").unwrap(), 1);
        assert_eq!(parse_first("aaoue2").unwrap(), 2);
        assert_eq!(parse_first("tht1three4").unwrap(), 1);
        assert_eq!(parse_first("abcdef").is_err(), true);
    }

    #[test]
    fn test_index_first_digit() {
        assert_eq!(index_first_digit("ab1").unwrap(), (2, 1));
        assert_eq!(index_first_digit("abwthua").is_none(), true);
    }

    #[test]
    fn test_index_first_letter() {
        assert_eq!(index_first_letter("aono"), None);
        assert_eq!(index_first_letter("aone").unwrap(), (1, 1));
        assert_eq!(index_first_letter("aaone2").unwrap(), (2, 1));
        assert_eq!(search_forwarad("aaone2", 2, "one"), true);
    }

    #[test]
    fn test_parse_with_digits_first() {
        assert_eq!(parse_with_digits("two1nine").unwrap(), 29);
        assert_eq!(parse_with_digits("eightwothree").unwrap(), 83);
        assert_eq!(parse_with_digits("abcone2threexyz").unwrap(), 13);
        assert_eq!(parse_with_digits("xtwone3four").unwrap(), 24);
        assert_eq!(parse_with_digits("4nineeightseven2").unwrap(), 42);
        assert_eq!(parse_with_digits("zoneight234").unwrap(), 14);
        assert_eq!(parse_with_digits("7pqrstsixteen").unwrap(), 76);
        assert_eq!(parse_with_digits("9hcj6nine").unwrap(), 99);
    }

    #[test]
    fn test_driver_result() {
        assert_eq!(driver().unwrap(), 54708);
    }

    #[test]
    fn test_parse_first_line() {
        assert_eq!(parse_line("1abc2").unwrap(), 12);
    }

    #[test]
    fn test_parse_second_line() {
        assert_eq!(parse_line("pqr3stu8vwx").unwrap(), 38);
    }
}
