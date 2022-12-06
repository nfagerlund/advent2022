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

fn real_played_shape(code: &str) -> Shape {
    match code {
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,
        _ => panic!("meh"),
    }
}

fn score_shape(shape: Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn score_outcome(outcome: Outcome) -> usize {
    match outcome {
        Outcome::Win => 6,
        Outcome::Lose => 0,
        Outcome::Draw => 3,
    }
}

fn score_matches(scoring: &HashMap<String, usize>, matches: &str) -> usize {
    matches.lines()
        .map(|match_text| scoring.get(match_text).unwrap())
        .fold(0, |total, round| total + *round)
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Scissors,
    Paper,
}

#[derive(Clone, Copy)]
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
        let outcome = real_outcome(outcome_code);
        let result_score = score_outcome(outcome);
        for &face_code in ["A", "B", "C"].iter() {
            let match_text = format!("{face_code} {outcome_code}");
            let face = real_faced_shape(face_code);
            let desired_shape = match (face, outcome) {
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
            let shape_score = score_shape(desired_shape);
            let score = shape_score + result_score;
            scoring.insert(match_text, score);
        }
    }

    score_matches(&scoring, inputs)
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
    for &play_code in ["X", "Y", "Z"].iter() {
        let play = real_played_shape(play_code);
        let shape_score: usize = score_shape(play);
        for &face_code in ["A", "B", "C"].iter() {
            let face = real_faced_shape(face_code);
            let match_text = format!("{face_code} {play_code}");
            let outcome = match (face, play) {
                (Shape::Rock, Shape::Rock) => Outcome::Draw,
                (Shape::Rock, Shape::Scissors) => Outcome::Lose,
                (Shape::Rock, Shape::Paper) => Outcome::Win,

                (Shape::Scissors, Shape::Rock) => Outcome::Win,
                (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
                (Shape::Scissors, Shape::Paper) => Outcome::Lose,

                (Shape::Paper, Shape::Rock) => Outcome::Lose,
                (Shape::Paper, Shape::Scissors) => Outcome::Win,
                (Shape::Paper, Shape::Paper) => Outcome::Draw,
            };
            let result_score = score_outcome(outcome);
            let score = shape_score + result_score;
            scoring.insert(match_text, score);
        }
    }

    score_matches(&scoring, inputs)
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
