use clap::Parser;
use std::io::{stdin, Read};

#[derive(Parser)]
struct Args {
    #[arg(short, long, action)]
    entry: bool,
}

#[derive(Debug, Default)]
struct State {
    sum: i64,
    pos: i64,
    basement_pos: Option<i64>,
}

fn main() {
    let args = Args::parse();
    let input = stdin();

    let result = input
        .lock()
        .bytes()
        .map(|i| match i {
            Ok(40) => 1,  // (
            Ok(41) => -1, // )
            _ => 0,
        })
        .fold(State::default(), |acc, i| State {
            sum: acc.sum + i,
            pos: acc.pos + 1,
            basement_pos: if acc.basement_pos.is_none() && (acc.sum + i < 0) {
                Some(acc.pos + 1)
            } else {
                acc.basement_pos
            },
        });

    if args.entry {
        println!(
            "Entering basement at step: {}",
            result.basement_pos.unwrap()
        );
    } else {
        println!("Final position: {}", result.sum);
    }
}
