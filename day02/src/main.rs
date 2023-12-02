//use clap::Parser;
use regex::Regex;
use std::io::stdin;
#[macro_use]
extern crate lazy_static;

#[derive(Debug, Default)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorSet {
    fn from_str(s: &str) -> ColorSet {
        lazy_static! {
            static ref RE_REVEAL: Regex = Regex::new(r"(\d+) (blue|red|green)(, |$)").unwrap();
        }

        let mut result = ColorSet::default();

        for i in RE_REVEAL.captures_iter(s) {
            let num = str::parse::<u32>(i.get(1).unwrap().as_str()).unwrap();
            let color = i.get(2).unwrap().as_str();
            match color {
                "red" => result.red += num,
                "green" => result.green += num,
                "blue" => result.blue += num,
                _ => unimplemented!(),
            }
        }

        result
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<ColorSet>,
}

impl Game {
    fn from_str(line: String) -> Game {
        lazy_static! {
            static ref RE_ID: Regex = Regex::new(r"^Game (\d+):").unwrap();
            static ref RE_REVEALS: Regex = Regex::new(r"[:;]([^;]+)").unwrap();
        }

        let id = RE_ID
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        let mut result = Game {
            id,
            reveals: vec![],
        };

        for reveal in RE_REVEALS.find_iter(&line) {
            result.reveals.push(ColorSet::from_str(reveal.as_str()));
        }

        result
    }

    fn minimal_set(&self) -> ColorSet {
        let mut result = ColorSet::default();
        for r in self.reveals.iter() {
            result.red = if r.red > result.red { r.red } else { result.red };
            result.green = if r.green > result.green { r.green } else { result.green };
            result.blue = if r.blue > result.blue { r.blue } else { result.blue };
        }
        result
    }
}

fn main() {
    let result = stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| Game::from_str(s))
        .map(|g| g.minimal_set())
        .fold(0, |acc, min_set| acc + min_set.power());
    println!("{}", result);
}
