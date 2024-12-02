/*
--- Day 2: Rock Paper Scissors ---
The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z
This strategy guide predicts and recommends the following:

In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

Your puzzle answer was 13924.

--- Part Two ---
The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

Your puzzle answer was 13448.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.


*/
use std::convert::TryFrom;
use std::error::Error;
use std::ops::Not;

#[derive(PartialEq, Eq)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Eq, PartialEq)]
enum Game {
    Win,
    Lose,
    Draw,
}

impl TryFrom<&str> for Game {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "X" => Ok(Game::Lose),
            "Y" => Ok(Game::Draw),
            "Z" => Ok(Game::Win),
            _ => Err("ParseError"),
        }
    }
}

impl Not for Game {
    type Output = Game;

    fn not(self) -> Self::Output {
        match self {
            Self::Win => Self::Lose,
            Self::Lose => Self::Win,
            Self::Draw => Self::Draw,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Hand {
    Rock = 1,
    Paper = 2,
    Sizzors = 3,
}

impl TryFrom<&str> for Hand {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Hand, Self::Error> {
        match string {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Sizzors),
            _ => Err("ParseError"),
        }
    }
}

impl From<&Hand> for u32 {
    fn from(hand: &Hand) -> u32 {
        match *hand {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Sizzors => 3,
        }
    }
}

impl Hand {
    fn cmp(&self, other: &Self) -> Game {
        match *self {
            Hand::Rock => match *other {
                Hand::Rock => Game::Draw,
                Hand::Paper => Game::Lose,
                Hand::Sizzors => Game::Win,
            },
            Hand::Paper => match *other {
                Hand::Rock => Game::Win,
                Hand::Paper => Game::Draw,
                Hand::Sizzors => Game::Lose,
            },
            Hand::Sizzors => match *other {
                Hand::Rock => Game::Lose,
                Hand::Paper => Game::Win,
                Hand::Sizzors => Game::Draw,
            },
        }
    }

    fn opponent_from_result(&self, game: &Game) -> &Hand {
        let result = (self, game);

        match result {
            (hand, Game::Draw) => hand,
            (&Hand::Rock, &Game::Win) | (&Hand::Paper, &Game::Lose) => &Hand::Sizzors,
            (&Hand::Sizzors, &Game::Win) | (&Hand::Rock, &Game::Lose) => &Hand::Paper,
            (&Hand::Paper, &Game::Win) | (&Hand::Sizzors, &Game::Lose) => &Hand::Rock,
        }
    }
}

fn score(theirs: &Hand, yours: &Hand) -> u32 {
    let score: u32 = yours.into();

    match yours.cmp(theirs) {
        Game::Win => score + 6,
        Game::Draw => score + 3,
        Game::Lose => score,
    }
}

fn score_line(line: &str, part: &Part) -> Result<u32, Box<dyn Error>> {
    let mut splits = line.split_whitespace();
    let theirs = Hand::try_from(splits.next().unwrap())?;

    match *part {
        Part::Part1 => {
            let yours = Hand::try_from(splits.next().unwrap())?;
            Ok(score(&theirs, &yours))
        }
        Part::Part2 => {
            let result = Game::try_from(splits.next().unwrap())?;
            let yours = theirs.opponent_from_result(&(!result));
            Ok(score(&theirs, yours))
        }
    }
}

fn score_lines(lines: &[&str], part: &Part) -> u32 {
    lines.iter().map(|l| score_line(l, part).unwrap()).sum()
}

fn main() {
    let input = include_str!("../data/input.txt");

    let lines: Vec<&str> = input.lines().collect();
    println!("Part 1 answer {}", score_lines(&lines, &Part::Part1));
    println!("Part 2 answer {}", score_lines(&lines, &Part::Part2));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_compare() {
        assert_eq!(Hand::Rock.cmp(&Hand::Paper), Game::Lose);
        assert_eq!(Hand::Paper.cmp(&Hand::Sizzors), Game::Lose);
        assert_eq!(Hand::Sizzors.cmp(&Hand::Rock), Game::Lose);
    }
    #[test]
    fn test_hand_from_str() -> Result<(), Box<dyn Error>> {
        assert_eq!(Hand::try_from("A")?, Hand::Rock);
        assert_eq!(Hand::try_from("B")?, Hand::Paper);
        assert_eq!(Hand::try_from("C")?, Hand::Sizzors);

        assert_eq!(Hand::try_from("X")?, Hand::Rock);
        assert_eq!(Hand::try_from("Y")?, Hand::Paper);
        assert_eq!(Hand::try_from("Z")?, Hand::Sizzors);

        Ok(())
    }

    #[test]
    fn test_game_from_str() -> Result<(), Box<dyn Error>> {
        assert_eq!(Game::try_from("X")?, Game::Lose);
        assert_eq!(Game::try_from("Y")?, Game::Draw);
        assert_eq!(Game::try_from("Z")?, Game::Win);
        Ok(())
    }
    #[test]
    fn test_score() {
        let theirs = Hand::Rock;
        let yours = Hand::Paper;
        assert_eq!(score(&theirs, &yours), 8);

        let theirs = Hand::Paper;
        let yours = Hand::Rock;
        assert_eq!(score(&theirs, &yours), 1);

        let theirs = Hand::Sizzors;
        let yours = Hand::Sizzors;
        assert_eq!(score(&theirs, &yours), 6);
    }

    #[test]
    fn test_other_hand() {
        assert_eq!(Hand::Rock.opponent_from_result(&Game::Win), &Hand::Sizzors);
        assert_eq!(Hand::Rock.opponent_from_result(&Game::Lose), &Hand::Paper);
        assert_eq!(Hand::Rock.opponent_from_result(&Game::Draw), &Hand::Rock);

        assert_eq!(Hand::Paper.opponent_from_result(&Game::Win), &Hand::Rock);
        assert_eq!(
            Hand::Paper.opponent_from_result(&Game::Lose),
            &Hand::Sizzors
        );
        assert_eq!(Hand::Paper.opponent_from_result(&Game::Draw), &Hand::Paper);

        assert_eq!(Hand::Sizzors.opponent_from_result(&Game::Win), &Hand::Paper);
        assert_eq!(Hand::Sizzors.opponent_from_result(&Game::Lose), &Hand::Rock);
        assert_eq!(
            Hand::Sizzors.opponent_from_result(&Game::Draw),
            &Hand::Sizzors
        );
    }

    #[test]
    fn test_score_line() -> Result<(), Box<dyn Error>> {
        assert_eq!(score_line("A Y", &Part::Part1)?, 8);
        assert_eq!(score_line("B X", &Part::Part1)?, 1);
        assert_eq!(score_line("C Z", &Part::Part1)?, 6);

        assert_eq!(score_line("A Y", &Part::Part2)?, 4);
        assert_eq!(score_line("B X", &Part::Part2)?, 1);
        assert_eq!(score_line("C Z", &Part::Part2)?, 7);

        Ok(())
    }

    #[test]
    fn test_score_lines_from_file() {
        let input = include_str!("../data/input_test.txt");
        let lines: Vec<&str> = input.lines().collect();
        assert_eq!(score_lines(&lines, &Part::Part1), 15);
        assert_eq!(score_lines(&lines, &Part::Part2), 12);
    }
}
