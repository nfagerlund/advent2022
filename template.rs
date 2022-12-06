use advent2022::*;

fn main() {
    let inputs = load_inputs("daySOMETHING").unwrap();
    println!("one: {}", part_one(&inputs));
    println!("two: {}", part_two(&inputs));
}

fn part_two(inputs: &str) -> () {}

fn part_one(inputs: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#""#;

    #[test]
    fn example_part_one() {
        let answer = ();
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }

    #[test]
    fn example_part_two() {
        let answer = ();
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
