use std::collections::{HashMap, HashSet};

use advent2022::*;

fn main() {
    let inputs = load_inputs("day3").unwrap();
    println!("one: {}", part_one(&inputs));
    println!("two: {}", part_two(&inputs));
}

fn build_priorities() -> HashMap<char, usize> {
    let mut stuff = HashMap::new();
    for (i, l) in ('a'..='z').enumerate() {
        stuff.insert(l, i + 1);
    }
    for (i, l) in ('A'..='Z').enumerate() {
        stuff.insert(l, i + 27);
    }
    stuff
}
struct Priorities;
impl Priorities {
    fn get(c: char) -> Option<usize> {
        if let Ok(n) = u8::try_from(c) {
            Priorities::get_byte(n)
        } else {
            None
        }
    }

    fn get_byte(b: u8) -> Option<usize> {
        let n = b as usize;
        if n > 96 && n < 123 {
            // lowercase a = 97
            Some(n - 96)
        } else if n > 64 && n < 91 {
            // capital A = 65
            Some(n - 38)
        } else {
            None
        }
    }
}

// I think I can use hashsets real dumbly for this one, too.
fn part_two(inputs: &str) -> usize {
    let mut lines = inputs.lines();
    let mut sum = 0_usize;
    while let (Some(first), Some(second), Some(third)) = (lines.next(), lines.next(), lines.next()) {
        let mut seen: HashSet<u8> = HashSet::new();
        for i in first.as_bytes().iter() {
            seen.insert(*i);
        }
        let mut repeated: HashSet<u8> = HashSet::new();
        for i in second.as_bytes().iter() {
            if seen.contains(i) {
                repeated.insert(*i);
            }
        }
        let everywhere = third.as_bytes().iter().find(|&i| { repeated.contains(i) }).unwrap();
        let priority = Priorities::get_byte(*everywhere).unwrap();
        sum += priority;
    }

    sum
}

fn part_one(inputs: &str) -> usize {
    inputs.lines().fold(0_usize, |sum, line| {
        let bytes = line.as_bytes();
        let len = bytes.len();
        let half = len / 2;
        let (first, second) = bytes.split_at(half);
        let mut seen: HashSet<u8> = HashSet::new();
        for &i in first.iter() {
            seen.insert(i);
        }
        let repeated = second.iter().find(|&i| { seen.contains(i) }).unwrap();
        let priority = Priorities::get_byte(*repeated).unwrap();
        sum + priority
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;

    #[test]
    fn example_part_one() {
        let answer = 157;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = 70;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn dumb() {
        for i in ('a'..='z').enumerate() {
            dbg!(&i);
        }
    }

    #[test]
    fn priorities() {
        let stuff = build_priorities();
        assert_eq!(*(stuff.get(&'z').unwrap()), 26);
        assert_eq!(*(stuff.get(&'Y').unwrap()), 51);
    }

    #[test]
    fn alt_priorities() {
        assert_eq!(Priorities::get('z').unwrap(), 26);
        assert_eq!(Priorities::get('Y').unwrap(), 51);
    }

    fn chardump(c: char) {
        let byte: u8 = c.try_into().unwrap();
        dbg!(&c, &byte);
    }

    #[test]
    fn dumber() {
        for &c in ['a', 'z', 'A', 'Z'].iter() {
            chardump(c);
        }
    }

    #[test]
    fn half() {
        let stuff = "aoeuhtns";
        let bytes = stuff.as_bytes();
        let len = bytes.len();
        let half = len / 2;
        let second_half = bytes.split_at(half).1;
        let first_char: char = second_half[0].into();
        assert_eq!(first_char, 'h');
    }
}
