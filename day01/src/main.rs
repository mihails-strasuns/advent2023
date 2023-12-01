use std::io::{stdin, Read};

fn main() {
    let input = stdin().lock().bytes().fold(
        0,
        |acc, i| match i as char {
            ')' => acc -= 1,
            '(' => acc += 1,
        }
    );
}
