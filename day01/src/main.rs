use onig::*;
use std::io::stdin;
#[macro_use]
extern crate lazy_static;

fn str_to_num(s: &str, pos: usize) -> u32 {
    if let Ok(num) = s[pos..pos + 1].parse::<u32>() {
        return num;
    }
    let tail = &s[pos..];
    if tail.starts_with("one") {
        1
    } else if tail.starts_with("two") {
        2
    } else if tail.starts_with("three") {
        3
    } else if tail.starts_with("four") {
        4
    } else if tail.starts_with("five") {
        5
    } else if tail.starts_with("six") {
        6
    } else if tail.starts_with("seven") {
        7
    } else if tail.starts_with("eight") {
        8
    } else if tail.starts_with("nine") {
        9
    } else {
        unimplemented!();
    }
}

fn first_and_last(line: &str) -> (u32, u32) {
    lazy_static! {
        static ref RE_NUMISH: Regex =
            Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    }
    let nums: Vec<u32> = RE_NUMISH
        .find_iter(line)
        .map(|(a, _)| str_to_num(line, a))
        .collect();
    (nums[0], nums[nums.len()-1])
}

fn main() {
    let result = stdin()
        .lines()
        .map(|s| first_and_last(&s.unwrap()))
        .map(|(a, b)| a * 10 + b)
        .fold(0, |acc, i| acc + i);
    println!("{}", result);
}
