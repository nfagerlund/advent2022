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
    for &play in ["A", "B", "C"].iter() {
        for &face in ["X", "Y", "Z"].iter() {
            let shape_score: usize;
            let result_score: usize;
            let match_text = format!("{play} {face}");
            match play {
                "A" => {
                    shape_score = 1;
                    result_score = match face {
                        "X" => 3,
                        "Y" => 0,
                        "Z" => 6,
                        _ => panic!("huh????"),
                    };
                },
                "B" => {
                    shape_score = 2;
                    result_score = match face {
                        "X" => 6,
                        "Y" => 3,
                        "Z" => 0,
                        _ => panic!("huh????"),
                    };
                },
                "C" => {
                    shape_score = 3;
                    result_score = match face {
                        "X" => 0,
                        "Y" => 6,
                        "Z" => 3,
                        _ => panic!("huh????"),
                    };
                },
                _ => panic!("lol wut"),
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
