use advent2022::*;
use std::collections::HashMap;

fn main() {
    let inputs = load_inputs("day2").unwrap();
    println!("total score after playing all rounds: {}", part_one(&inputs));
    // println!("two: {}", part_two(&inputs));
}

fn part_two(inputs: &str) -> () {}

// a, x = rock (1 point)
// b, y = paper (2 points)
// c, z = scissors (3 points)
// lose = 0, draw = 3, win = 6
// score = shape + result
/// total score when playing provided strategy every round
fn part_one(inputs: &str) -> usize {
    // wait... let's just precompute all the possible round scores, it's only
    // rock paper scissors after all. Then we only do the math once.
    let mut scoring: HashMap<String, usize> = HashMap::new();
    for &play in ["X", "Y", "Z"].iter() {
        let shape_score: usize = match play {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!("huh?!?!"),
        };
        for &face in ["A", "B", "C"].iter() {
            let match_text = format!("{face} {play}");
            let result_score: usize = match play {
                "X" => {
                    match face {
                        "A" => 3,
                        "B" => 0,
                        "C" => 6,
                        _ => panic!("no"),
                    }
                },
                "Y" => {
                    match face {
                        "A" => 6,
                        "B" => 3,
                        "C" => 0,
                        _ => panic!("no"),
                    }
                },
                "Z" => {
                    match face {
                        "A" => 0,
                        "B" => 6,
                        "C" => 3,
                        _ => panic!("no"),
                    }

                },
                _ => panic!("nah!!"),
            };
            let score = shape_score + result_score;
            scoring.insert(match_text, score);
        }
    }

    inputs.lines()
        .map(|match_text| scoring.get(match_text).unwrap())
        .fold(0, |total, round| total + *round)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn example_part_one() {
        let answer = 15;
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
