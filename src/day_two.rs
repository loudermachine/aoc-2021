use std::fs;

enum CommandType {
    Forward,
    Down,
    Up,
}

impl From<&str> for CommandType {
    fn from(s: &str) -> Self {
        match s {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => panic!("Invalid command type")
        }
    }
}

struct Command {
    r#type: CommandType,
    units: i32
}

#[derive(Debug)]
struct Position {
    pub horizontal: i32,
    pub depth: i32,
    pub aim: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { horizontal: 0, depth: 0, aim: 0 }
    }

    pub fn add(&mut self, cmd: &Command) {
        use CommandType::*;
        match cmd.r#type {
            Forward => {
                self.horizontal += cmd.units;
                self.depth += self.aim * cmd.units;
            },
            Up => {
                self.aim -= cmd.units
            },
            Down => {
                self.aim += cmd.units
            },

        }
    }
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        match s.split_whitespace().collect::<Vec<&str>>()[..] {
            [cmd, unit] => Command {
                r#type: CommandType::from(cmd),
                units: unit.parse::<i32>().unwrap()
            },
            _ => panic!("Invalid command")
        }
    }
}

fn read_commands() -> Vec<Command> {
    let input = fs::read_to_string("./data/day_two.txt").unwrap();
    input
        .split("\n")
        .map(|line| Command::from(line))
        .collect::<Vec<Command>>()
}

#[test]
fn part_one() {
    let cmds = read_commands();
    let (h_pos, d_pos) = cmds.iter()
        .fold((0, 0), |acc, cmd| match cmd.r#type {
            CommandType::Up => (acc.0, acc.1 - cmd.units),
            CommandType::Down => (acc.0, acc.1 + cmd.units),
            CommandType::Forward => (acc.0 + cmd.units, acc.1)
        });

    println!(
         "What do you get if you multiply your final horizontal position by your final depth?\nR: {}", h_pos * d_pos
    )
}

#[test]
fn part_two() {
    let cmds = read_commands();
    let mut pos = Position::new();
    cmds.iter().for_each(|cmd| pos.add(cmd));
    println!(
        "What do you get if you multiply your final horizontal position by your final depth?\nR: {}", pos.horizontal * pos.depth
   )
}