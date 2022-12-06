use advent2022::*;

fn main() {
    let inputs = load_inputs("day1").unwrap();
    println!("max calories carried by a single elf: {}", part_one(&inputs));
}

fn part_one(inputs: &str) -> usize {
    let totals: Vec<usize> = inputs.split("\n\n").map(|counts| {
        counts.lines().fold(0, |total, line| {
            total + usize_or_die(line)
        })
    }).collect();
    let max = totals.iter().max().unwrap();
    *max
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn example_part_one() {
        let answer = 24000;
        let result = part_one(EXAMPLE);
        assert_eq!(result, answer);
    }
}
