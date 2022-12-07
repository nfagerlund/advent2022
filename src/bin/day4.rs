use std::ops::RangeInclusive;

use advent2022::*;

fn main() {
    let inputs = load_inputs("day4").unwrap();
    println!("one: {}", part_one(&inputs));
    println!("two: {}", part_two(&inputs));
}

fn part_two(inputs: &str) -> usize {
    0
}

fn part_one(inputs: &str) -> usize {
    let range_pairs = inputs.lines().map(|line| {
        line
            .split(",")
            .map(|range_str| {
                let mut ends = range_str.split("-");
                let start = usize_or_die(ends.next().unwrap());
                let end = usize_or_die(ends.next().unwrap());
                assert_eq!(None, ends.next());
                RangeInclusive::new(start, end)
            })
    });
    // filter_map because its closure takes ownership of the iterated values.
    let count = range_pairs.filter_map(|mut pair_iter| {
        let first = pair_iter.next().unwrap();
        let second = pair_iter.next().unwrap();
        // assert_eq!(None, pair_iter.next());
        if (first.contains(second.start()) && first.contains(second.end())) || (second.contains(first.start()) && second.contains(first.end())) {
            Some(())
        } else {
            None
        }
    }).count();

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;

    #[test]
    fn example_part_one() {
        let answer = 2;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        // assert_eq!(result, answer);
    }
}
