/*
--- Day 5: Supply Stacks ---
The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3
Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3
Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3
The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?

Your puzzle answer was ZRLJGSCTR.

--- Part Two ---
As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3
However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3
Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3
Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3
In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?

Your puzzle answer was PRTTGRFPB.

Both parts of this puzzle are complete! They provide two gold stars: **

At this point, you should return to your Advent calendar and try another puzzle.

If you still want to see it, you can get your puzzle input.

*/
use regex::Regex;

#[derive(Debug, Default)]
struct Stack {
    values: Vec<char>,
}

impl Stack {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }

    pub fn push(&mut self, value: char) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> char {
        self.values.pop().unwrap()
    }

    pub fn top(&self) -> &char {
        self.values.last().unwrap()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

enum Order {
    Retain,
    Reverse,
}

fn move_n(stacks: &mut [Stack], source_idx: usize, target_idx: usize, count: usize, order: &Order) {
    match order {
        Order::Retain => {
            let mut temp_stack = Stack::new();
            for _ in 0..count {
                let t_val = stacks[source_idx].pop();
                temp_stack.push(t_val);
            }
            for _ in 0..count {
                let t_val = temp_stack.pop();
                stacks[target_idx].push(t_val);
            }
        }
        Order::Reverse => {
            for _ in 0..count {
                let t_val = stacks[source_idx].pop();
                stacks[target_idx].push(t_val);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command {
    pub num_items: usize,
    pub source_idx_base_1: usize,
    pub target_idx_base_1: usize,
}

fn parse_line(line: &str) -> Option<Command> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    if let Some(cap) = re.captures_iter(line).next() {
        return Some(Command {
            num_items: String::from(&cap[1]).parse().unwrap(),
            source_idx_base_1: String::from(&cap[2]).parse().unwrap(),
            target_idx_base_1: String::from(&cap[3]).parse().unwrap(),
        });
    }

    None
}

fn initialize_stacks() -> Vec<Stack> {
    let mut s = Vec::new();

    for _ in 0..9 {
        s.push(Stack::new());
    }

    for c in "STHFWR".chars() {
        s[0].push(c);
    }

    for c in "SGDQW".chars() {
        s[1].push(c);
    }

    for c in "BTW".chars() {
        s[2].push(c);
    }

    for c in "DRWTNQZJ".chars() {
        s[3].push(c);
    }

    for c in "FBHGLVTZ".chars() {
        s[4].push(c);
    }

    for c in "LPTCVBSG".chars() {
        s[5].push(c);
    }

    for c in "ZBRTWGP".chars() {
        s[6].push(c);
    }

    for c in "NGMTCJR".chars() {
        s[7].push(c);
    }

    for c in "LGBW".chars() {
        s[8].push(c);
    }

    s
}

fn part1() -> String {
    let input = include_str!("../data/input.txt");
    let mut stacks = initialize_stacks();

    for line in input.lines().map(parse_line) {
        match line {
            Some(command) => {
                move_n(
                    &mut stacks,
                    command.source_idx_base_1 - 1,
                    command.target_idx_base_1 - 1,
                    command.num_items,
                    &Order::Reverse,
                );
            }
            None => {
                continue;
            }
        }
    }

    stacks.iter().map(Stack::top).collect()
}

fn part2() -> String {
    let input = include_str!("../data/input.txt");
    let mut stacks = initialize_stacks();

    for line in input.lines().map(parse_line) {
        match line {
            Some(command) => {
                move_n(
                    &mut stacks,
                    command.source_idx_base_1 - 1,
                    command.target_idx_base_1 - 1,
                    command.num_items,
                    &Order::Retain,
                );
            }
            None => {
                continue;
            }
        }
    }

    stacks.iter().map(Stack::top).collect()
}

fn main() {
    println!("Part1 answer {}", part1());
    println!("Part2 answer {}", part2());
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn initialize_test_stacks() -> Vec<Stack> {
        let mut s = vec![Stack::new(), Stack::new(), Stack::new()];

        s[0].push('Z');
        s[0].push('N');

        s[1].push('M');
        s[1].push('C');
        s[1].push('D');

        s[2].push('P');

        s
    }

    #[test]
    fn test_basic_stack_operations() {
        let mut stacks = initialize_test_stacks();

        let v = stacks[1].pop();
        stacks[0].push(v);

        assert_eq!(stacks[0].top(), &'D');
        assert_eq!(stacks[1].top(), &'C');
    }

    #[test]
    fn test_move_n_reverse() {
        let mut stacks = initialize_test_stacks();

        move_n(&mut stacks, 1, 0, 1, &Order::Reverse);

        assert_eq!(stacks[0].top(), &'D');
        assert_eq!(stacks[1].top(), &'C');
        assert_eq!(stacks[2].top(), &'P');

        move_n(&mut stacks, 0, 2, 3, &Order::Reverse);

        assert!(stacks[0].is_empty());
        assert_eq!(stacks[1].top(), &'C');
        assert_eq!(stacks[2].top(), &'Z');

        move_n(&mut stacks, 1, 0, 2, &Order::Reverse);

        assert_eq!(stacks[0].top(), &'M');
        assert!(stacks[1].is_empty());
        assert_eq!(stacks[2].top(), &'Z');

        move_n(&mut stacks, 0, 1, 1, &Order::Reverse);

        assert_eq!(stacks[0].top(), &'C');
        assert_eq!(stacks[1].top(), &'M');
        assert_eq!(stacks[2].top(), &'Z');
    }

    #[test]
    fn test_move_n_retain() {
        let mut stacks = initialize_test_stacks();

        move_n(&mut stacks, 1, 0, 1, &Order::Retain);

        assert_eq!(stacks[0].top(), &'D');
        assert_eq!(stacks[1].top(), &'C');
        assert_eq!(stacks[2].top(), &'P');

        move_n(&mut stacks, 0, 2, 3, &Order::Retain);

        assert!(stacks[0].is_empty());
        assert_eq!(stacks[1].top(), &'C');
        assert_eq!(stacks[2].top(), &'D');

        move_n(&mut stacks, 1, 0, 2, &Order::Retain);

        assert_eq!(stacks[0].top(), &'C');
        assert!(stacks[1].is_empty());
        assert_eq!(stacks[2].top(), &'D');

        move_n(&mut stacks, 0, 1, 1, &Order::Retain);

        assert_eq!(stacks[0].top(), &'M');
        assert_eq!(stacks[1].top(), &'C');
        assert_eq!(stacks[2].top(), &'D');
    }

    #[test]
    fn test_parse_line() {
        let lines = r#"
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "#;

        let lines: Vec<&str> = lines.trim().lines().collect();

        assert_eq!(
            parse_line(lines[0]).unwrap(),
            Command {
                num_items: 1,
                source_idx_base_1: 2,
                target_idx_base_1: 1
            }
        );
        assert_eq!(
            parse_line(lines[1]).unwrap(),
            Command {
                num_items: 3,
                source_idx_base_1: 1,
                target_idx_base_1: 3
            }
        );
        assert_eq!(
            parse_line(lines[2]).unwrap(),
            Command {
                num_items: 2,
                source_idx_base_1: 2,
                target_idx_base_1: 1
            }
        );
        assert_eq!(
            parse_line(lines[3]).unwrap(),
            Command {
                num_items: 1,
                source_idx_base_1: 1,
                target_idx_base_1: 2
            }
        );
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../data/input-test.txt");
        let mut stacks = initialize_test_stacks();

        for line in input.lines().map(parse_line) {
            match line {
                Some(command) => {
                    move_n(
                        &mut stacks,
                        command.source_idx_base_1 - 1,
                        command.target_idx_base_1 - 1,
                        command.num_items,
                        &Order::Reverse,
                    );
                }
                None => {
                    continue;
                }
            }
        }

        let result: String = stacks.iter().map(Stack::top).collect();
        assert_eq!(result, "CMZ".to_owned());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../data/input-test.txt");
        let mut stacks = initialize_test_stacks();

        for line in input.lines().map(parse_line) {
            match line {
                Some(command) => {
                    move_n(
                        &mut stacks,
                        command.source_idx_base_1 - 1,
                        command.target_idx_base_1 - 1,
                        command.num_items,
                        &Order::Retain,
                    );
                }
                None => {
                    continue;
                }
            }
        }

        let result: String = stacks.iter().map(Stack::top).collect();
        assert_eq!(result, "MCD".to_owned());
    }
}
