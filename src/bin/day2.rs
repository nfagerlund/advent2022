use advent2022::*;
use std::collections::HashMap;

fn main() {
    let inputs = load_inputs("day2").unwrap();
    println!("total score after playing all rounds: {}", part_one(&inputs));
    println!("two: {}", part_two(&inputs));
}

fn real_outcome(code: &str) -> Outcome {
    match code {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("bye"),
    }
}

fn real_faced_shape(code: &str) -> Shape {
    match code {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("meh"),
    }
}

enum Shape {
    Rock,
    Scissors,
    Paper,
}

enum Outcome {
    Win,
    Lose,
    Draw,
}

fn part_two(inputs: &str) -> usize {
    // Ok, exact same idea, but.
    let mut scoring: HashMap<String, usize> = HashMap::new();
    for &outcome_code in ["X", "Y", "Z"].iter() {
        // x => lose, y => draw, z => win
        for &face_code in ["A", "B", "C"].iter() {
            let match_text = format!("{face_code} {outcome_code}");
            let result_score: usize = match real_outcome(outcome_code) {
                Outcome::Lose => 0,
                Outcome::Draw => 3,
                Outcome::Win => 6,
            };
            let desired_shape = match (real_faced_shape(face_code), real_outcome(outcome_code)) {
                (Shape::Paper, Outcome::Win) => Shape::Scissors,
                (Shape::Paper, Outcome::Lose) => Shape::Rock,
                (Shape::Paper, Outcome::Draw) => Shape::Paper,

                (Shape::Rock, Outcome::Win) => Shape::Paper,
                (Shape::Rock, Outcome::Lose) => Shape::Scissors,
                (Shape::Rock, Outcome::Draw) => Shape::Rock,

                (Shape::Scissors, Outcome::Win) => Shape::Rock,
                (Shape::Scissors, Outcome::Lose) => Shape::Paper,
                (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            };
            let shape_score: usize = match desired_shape {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            let score = shape_score + result_score;
            scoring.insert(match_text, score);
        }
    }

    inputs.lines()
        .map(|match_text| scoring.get(match_text).unwrap())
        .fold(0, |total, round| total + *round)
}

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
        let answer = 12;
        let result = part_two(EXAMPLE);
        assert_eq!(result, answer);
    }
}
