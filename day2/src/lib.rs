use anyhow::{anyhow, Result};
use std::{fs, ops::Add};

static MAX_RED: usize = 12;
static MAX_GREEN: usize = 13;
static MAX_BLUE: usize = 14;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    fn from_line(line: &str) -> Result<Self> {
        let mut game = Game::default();
        for (i, part) in line.split(":").enumerate() {
            match i {
                0 => {
                    // Case: Header
                    game.id = part
                        .split(" ")
                        .last()
                        .unwrap_or("0")
                        .to_string()
                        .parse::<usize>()?;
                }
                1 => {
                    // Case: Draws
                    for draw in part.split(";") {
                        let d = Draw::from_input(draw)?;
                        game.draws.push(d);
                    }
                }
                _ => {
                    // Case: impossible
                    break;
                }
            }
        }
        Ok(game)
    }

    fn validate(&self) -> bool {
        for d in &self.draws {
            if !d.validate() {
                return false;
            }
        }
        true
    }

    fn minimum(&self) -> Draw {
        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for draw in &self.draws {
            if draw.red > min_red {
                min_red = draw.red;
            }
            if draw.green > min_green {
                min_green = draw.green;
            }
            if draw.blue > min_blue {
                min_blue = draw.blue;
            }
        }

        Draw {
            green: min_green,
            blue: min_blue,
            red: min_red,
        }
    }
}

impl Add for Game {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut g = Game::default();
        g.id = self.id + rhs.id;
        g
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Draw {
    blue: usize,
    red: usize,
    green: usize,
}

impl Draw {
    fn power(&self) -> usize {
        self.blue * self.red * self.green
    }

    fn from_input(line: &str) -> Result<Self> {
        let mut draw = Draw::default();
        for part in line.split(",") {
            if part.contains("blue") {
                draw.blue = Self::parse_inner(part)?;
            } else if part.contains("green") {
                draw.green = Self::parse_inner(part)?;
            } else if part.contains("red") {
                draw.red = Self::parse_inner(part)?;
            }
        }

        Ok(draw)
    }

    fn parse_inner(part: &str) -> Result<usize> {
        Ok(part
            .trim()
            .split(" ")
            .nth(0)
            .unwrap_or("0")
            .to_string()
            .parse::<usize>()?)
    }

    fn validate(&self) -> bool {
        self.red <= MAX_RED && self.blue <= MAX_BLUE && self.green <= MAX_GREEN
    }
}

pub fn driver() -> Result<usize> {
    let cont = fs::read_to_string("input.txt")?;
    Ok(cont
        .split("\n")
        .map(|x| {
            let g = Game::from_line(x).unwrap_or(Game::default());
            g
        })
        .filter(|x| x.validate())
        .map(|x| {
            // println!("{:?}", x);
            x
        })
        .reduce(|a, x| a + x)
        .unwrap()
        .id)
}

pub fn driver_two() -> Result<usize> {
    let cont = fs::read_to_string("input.txt")?;
    let b = cont
        .split("\n")
        .map(|x| Game::from_line(x).unwrap_or(Game::default()))
        .map(|x| x.minimum())
        .fold(0, |acc, x| acc + x.power());

    Ok(b)
}

mod tests {
    use super::*;

    #[test]
    fn test_minimum_example() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let g = Game::from_line(line).unwrap();
        assert_eq!(g.minimum().power(), 48);
    }

    #[test]
    fn test_complete() {
        assert_eq!(driver().unwrap(), 2679);
    }

    #[test]
    fn test_complete_two() {
        assert_eq!(driver_two().unwrap(), 77607);
    }

    #[test]
    fn test_example_complete() {
        let test = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let test = test
            .iter()
            .map(|x| {
                let g = Game::from_line(x).unwrap();
                // println!("INNER: {:?}", g);
                g
            })
            .filter(|x| x.validate());

        let mut sum = 0;
        for t in test {
            // println!("FLITRD: {:?}", t);
            sum += t.id;
        }

        assert_eq!(sum, 8);
    }

    #[test]
    fn test_filter() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let draw = Game::from_line(line).unwrap();
        println!("Game: {:?}", draw);
        assert_eq!(draw.validate(), false);
    }

    #[test]
    fn test_large_id() {
        let line = "Game 10: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_line(line).unwrap();
        assert_eq!(game.id, 10);
    }

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::from_line(line).unwrap();
        assert_eq!(game.id, 1);
        assert_eq!(game.draws.len(), 3);
        assert_eq!(game.draws.get(0).unwrap().blue, 3);
    }
}
