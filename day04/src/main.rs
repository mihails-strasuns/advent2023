use std::io::stdin;

#[derive(Default, Debug)]
struct Card {
    winning_nums: Vec<u64>,
    nums: Vec<u64>,
}

impl Card {
    fn from_str(line: &str) -> Card {
        let start = line.find(':').unwrap();
        let separator = line.find('|').unwrap();
        let mut result = Card::default();

        fn str_to_nums(s: &str) -> Vec<u64> {
            s.trim()
                .split(' ')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect()
        }
        result.winning_nums = str_to_nums(&line[start + 1..separator]);
        result.nums = str_to_nums(&line[separator + 1..]);

        result
    }

    fn matches(self) -> u64 {
        self.nums
            .iter()
            .map(|num| self.winning_nums.contains(num))
            .fold(0, |acc, matches| if matches { acc + 1 } else { acc })
    }
}

#[derive(Debug, Default)]
struct Tracker {
    copies: Vec<u64>,
    total: u64,
}

impl Tracker {
    fn new() -> Tracker {
        let mut result = Tracker::default();
        result.copies.resize(25, 1);
        result
    }

    fn next(&mut self) {
        self.total += self.copies[0];
        self.copies.rotate_left(1);
        let len = self.copies.len();
        self.copies[len-1] = 1;
    }
}

fn main() {
    let mut tracker = Tracker::new();

    for card_matches in stdin()
        .lines()
        .map(|s| Card::from_str(&s.unwrap()))
        .map(Card::matches)
    {
        println!("{}, {:?}", card_matches, tracker);
        assert!(card_matches <= 25);

        if tracker.copies[0] == 0 {
            break;
        }

        for i in 1..(1 + card_matches as usize) {
            tracker.copies[i] += tracker.copies[0];
        }

        tracker.next();
    }

    println!("{}", tracker.total);
}
