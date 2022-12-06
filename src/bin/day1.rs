use advent2022::*;

fn main() {
    let inputs = load_inputs("day1").unwrap();
    println!("max calories carried by a single elf: {}", part_one(&inputs));
    println!("total calories carried by top three elves: {}", part_two(&inputs));
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

fn part_two(inputs: &str) -> usize {
    // OK, let's get cheeky on this one. I would like to do this in one pass! No sorting!
    // vv (in desending order)
    let top_three: (usize, usize, usize) = inputs.split("\n\n")
        .map(|count_strs| {
            count_strs.lines().fold(0, |total, line| {
                total + usize_or_die(line)
            })
        })
        .fold((0,0,0), |leaders, total| {
            if total > leaders.0 {
                (total, leaders.0, leaders.1)
            } else if total > leaders.1 {
                (leaders.0, total, leaders.1)
            } else if total > leaders.2 {
                (leaders.0, leaders.1, total)
            } else {
                leaders
            }
        });
    top_three.0 + top_three.1 + top_three.2
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

    #[test]
    fn example_part_two() {
        let answer = 45000;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
