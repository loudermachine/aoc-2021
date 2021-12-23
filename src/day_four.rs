//! Returing the board index instead of a reference may be lame but it works.

#[derive(Clone)]
struct Cell {
    number: i32,
    flagged: bool,
}

impl Cell {
    pub fn new(number: i32) -> Self {
        Self {
            number,
            flagged: false,
        }
    }

    pub fn check(&mut self) {
        self.flagged = true
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn insert_row(&mut self, row: &str) {
        let mut row = row
            .split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|n| Cell::new(n.parse::<i32>().unwrap()))
            .collect::<Vec<Cell>>();
        self.cells.append(&mut row)
    }

    /// Returns a boolean indicating if a cell was checked
    pub fn check_cell(&mut self, number: i32) -> bool {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.number == number) {
            cell.flagged = true;
            return true;
        }

        false
    }

    pub fn won(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    fn check_rows(&self) -> bool {
        let rows = [(0, 5), (5, 10), (10, 15), (15, 20), (20, 25)];
        for (row_s, row_e) in rows {
            if self.cells[row_s..row_e].iter().all(|c| c.flagged) {
                return true;
            }
        }

        false
    }

    fn check_columns(&self) -> bool {
        for col in 0..5 {
            let iter = self.cells.iter().skip(col).step_by(5);
            let mut all_checked = true;
            for cell in iter {
                if !cell.flagged {
                    all_checked = false
                }
            }

            if all_checked {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        self.cells
            .iter()
            .fold(0, |sum, c| if c.flagged { sum } else { sum + c.number })
    }
}

struct Game {
    boards: Vec<Board>,
    inputs: Vec<i32>,
}

impl Game {
    pub fn parse(s: &str) -> Self {
        let mut lines = s.split('\n');
        let draw_numbers = lines.next().unwrap();
        let draw_numbers: Vec<i32> = draw_numbers
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let mut boards = Vec::new();
        while let Some(line) = lines.next() {
            if line == "\n" {
                continue;
            }

            let mut board = Board::new();
            for _ in 0..5 {
                match lines.next() {
                    Some(line) => board.insert_row(line),
                    None => unreachable!(),
                }
            }
            boards.push(board)
        }

        Self {
            boards,
            inputs: draw_numbers,
        }
    }

    pub fn run(&mut self) -> (i32, usize) {
        for number in self.inputs.iter() {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                if board.check_cell(*number) && board.won() {
                    return (*number, idx);
                }
            }
        }

        unreachable!()
    }

    pub fn run_until_last(&mut self) -> (i32, usize) {
        let mut last_won = None;
        for number in self.inputs.iter() {
            for (idx, board) in self.boards.iter_mut().enumerate() {
                if board.won() {
                    continue;
                }
                if board.check_cell(*number) && board.won() {
                    last_won.replace((*number, idx));
                }
            }

            if self.boards.iter().all(|b| b.won()) {
                break;
            }
        }

        last_won.unwrap()
    }
}

#[test]
fn day_four_part_one() {
    let input = std::fs::read_to_string("./data/day_four.txt").unwrap();

    let mut game = Game::parse(&input);
    let (ended, idx) = game.run();
    let sum = game.boards[idx].unmarked_sum();
    println!(
        "Game ended at: {}\n Unmarked sum: {}\n Final score: {}\n",
        ended,
        sum,
        sum * ended
    );
}

#[test]
fn day_four_part_two() {
    let input = std::fs::read_to_string("./data/day_four.txt").unwrap();

    let mut game = Game::parse(&input);
    let (ended, idx) = game.run_until_last();
    let sum = game.boards[idx].unmarked_sum();
    println!(
        "Game ended at: {}\n Unmarked sum: {}\n Final score: {}\n",
        ended,
        sum,
        sum * ended
    );
}

#[test]
fn day_four_part_one_example() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
    
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;

    let mut game = Game::parse(input);
    let (ended, idx) = game.run();
    let sum = game.boards[idx].unmarked_sum();
    assert_eq!(4512, sum * ended);
}

#[test]
fn day_four_part_two_example() {
    let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
    
 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6
    
14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;

    let mut game = Game::parse(input);
    let (ended, idx) = game.run_until_last();
    let sum = game.boards[idx].unmarked_sum();
    assert_eq!(1924, sum * ended);
}
