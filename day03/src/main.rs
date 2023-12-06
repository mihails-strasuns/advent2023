use ascii::{AsciiChar, AsciiStr, AsciiString};
use std::collections::HashMap;
use std::io::stdin;
use std::str::FromStr;

struct Schematic {
    lines: Vec<AsciiString>,
    gears: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

fn limit_l(value: i64, l: usize) -> usize {
    if value < l as i64 {
        l
    } else {
        value as usize
    }
}

fn limit_h(value: i64, h: usize) -> usize {
    if value > h as i64 {
        h
    } else {
        value as usize
    }
}

impl Schematic {
    fn has_adjacent_sym(
        gears: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
        lines: &Vec<AsciiString>,
        x: usize,
        y: usize,
        len: usize,
    ) -> bool {
        let x_low = limit_l(x as i64 - 1, 0);
        let x_high = limit_h(x as i64 + 2, lines.len() - 1);
        let y_low = limit_l(y as i64 - 1, 0);
        let y_high = limit_h((y + len + 1) as i64, lines[x].len() - 1);

        let mut result = false;

        for i in x_low..x_high {
            for j in y_low..y_high {
                let line = &lines[i];
                if line[j] == AsciiChar::Asterisk {
                    if let Some(v) = gears.get_mut(&(i, j)) {
                        v.push((x, y));
                    } else {
                        gears.insert((i, j), vec![(x, y)]);
                    }
                }
                if !line[j].as_char().is_numeric() && line[j] != '.' {
                    result = true;
                }
            }
        }

        result
    }

    fn find_next_num(s: &AsciiStr) -> Option<(usize, usize)> {
        let x = s.as_str().find(|c: char| c.is_numeric())?;
        let mut y = s[x..].as_str().find(|c: char| !c.is_numeric());
        if y.is_none() {
            y = Some(s.len())
        }
        Some((x, x + y.unwrap()))
    }

    fn sum_part_numbers(&mut self) -> u64 {
        let mut result: u64 = 0;
        for (line_idx, line) in self.lines.iter().enumerate() {
            let mut last_idx = 0;
            while let Some((mut a, mut b)) = Schematic::find_next_num(&line[last_idx..]) {
                a = a + last_idx;
                b = limit_h((b + last_idx) as i64, line.len());
                print!("Found number: {}", line[a..b].as_str());
                if Schematic::has_adjacent_sym(&mut self.gears, &self.lines, line_idx, a, b - a) {
                    println!(" -> part num");
                    result += str::parse::<u64>(line[a..b].as_str()).unwrap();
                } else {
                    println!()
                }
                last_idx = b;
            }
        }
        result
    }
}

fn main() {
    let mut result = Schematic {
        lines: stdin()
            .lines()
            .map(|s| AsciiString::from_str(&s.unwrap()).unwrap())
            .collect(),
        gears: HashMap::new(),
    };

    println!("{}", result.sum_part_numbers());
    let mut sum = 0;
    for num_vec in result.gears.values() {
        if num_vec.len() != 2 {
            continue;
        }

        sum += num_vec
            .iter()
            .map(|(x, y)| {
                let s = result.lines[*x][*y..].as_str();
                let end = s.find(|c| !char::is_numeric(c)).unwrap_or(s.len());
                let s = &s[0..end];
                str::parse::<u64>(s).unwrap()
            })
            .fold(1, |acc, num| acc * num);
    }
    println!("{}", sum);
}
