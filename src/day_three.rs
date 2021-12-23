//! Existe sem dúvida soluções mais elegantes do que essa, pois uma operação
//! (ex. gamma rate) é o inverso da outra (ex. epilson rate), mas é mais fácil
//! só copiar e colar o código e inverter as operações!  

use std::fs;

fn bit_at(n: i32, pos: u32) -> bool {
    let pos = 1 << pos;
    n & pos != 0
}

fn bit_count(nums: &[i32], pos: u32) -> (i32, i32) {
    nums.iter().fold((0, 0), |acc, n| {
        if bit_at(*n, pos) {
            (acc.0, acc.1 + 1)
        } else {
            (acc.0 + 1, acc.1)
        }
    })
}

#[derive(Debug)]
struct Diagnostic {
    reports: Vec<i32>,
    bit_len: u32,
}

impl Diagnostic {
    pub fn new(reports: Vec<i32>) -> Self {
        Self {
            reports,
            bit_len: 5,
        }
    }

    pub fn load() -> Self {
        let input = fs::read_to_string("./data/day_three.txt").unwrap();
        let reports = input
            .split('\n')
            .map(|line| i32::from_str_radix(line, 2).unwrap())
            .collect::<Vec<i32>>();

        Self {
            reports,
            bit_len: 12,
        }
    }

    pub fn power_consumption(&self) -> u32 {
        self.gamma_rate() * self.epilson_rate()
    }

    pub fn oxygen_gen_rating(&self) -> i32 {
        let mut candidates = self.reports.clone();
        for pos in (0..self.bit_len).rev() {
            let (zero_bits, one_bits) = bit_count(&candidates, pos);
            if one_bits >= zero_bits {
                candidates = candidates.into_iter().filter(|n| bit_at(*n, pos)).collect();
            } else {
                candidates = candidates
                    .into_iter()
                    .filter(|n| !bit_at(*n, pos))
                    .collect();
            }

            if candidates.len() == 1 {
                break;
            }
        }

        candidates.pop().unwrap()
    }

    pub fn co2_scrubber_rating(&self) -> i32 {
        let mut candidates = self.reports.clone();
        for pos in (0..self.bit_len).rev() {
            let (zero_bits, one_bits) = bit_count(&candidates, pos);
            if one_bits >= zero_bits {
                candidates = candidates
                    .into_iter()
                    .filter(|n| !bit_at(*n, pos))
                    .collect();
            } else {
                candidates = candidates.into_iter().filter(|n| bit_at(*n, pos)).collect();
            }

            if candidates.len() == 1 {
                break;
            }
        }

        candidates.pop().unwrap()
    }

    fn gamma_rate(&self) -> u32 {
        let mut rate = 0;
        for pos in (0..self.bit_len).rev() {
            let (zero_bits, one_bits) = bit_count(&self.reports, pos);
            if one_bits > zero_bits {
                rate |= 1 << pos
            }
        }

        rate
    }

    fn epilson_rate(&self) -> u32 {
        let mut rate = 0;
        for pos in (0..self.bit_len).rev() {
            let (zero_bits, one_bits) = bit_count(&self.reports, pos);
            if one_bits < zero_bits {
                rate |= 1 << pos
            }
        }

        rate
    }
}

#[test]
fn part_one() {
    let diag = Diagnostic::load();
    println!(
        "What is the power consumption of the submarine?\nR: {}",
        diag.power_consumption()
    )
}

#[test]
fn part_two() {
    let diag = Diagnostic::load();
    println!(
        "What is the life support rating of the submarine?\nR: {}",
        diag.oxygen_gen_rating() * diag.co2_scrubber_rating()
    )
}

#[test]
fn part_one_example() {
    let inputs = vec![
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let diag = Diagnostic::new(inputs);
    assert_eq!(diag.power_consumption(), 198)
}

#[test]
fn part_two_example() {
    let inputs = vec![
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];
    let diag = Diagnostic::new(inputs);
    assert_eq!(diag.oxygen_gen_rating(), 23);
    assert_eq!(diag.co2_scrubber_rating(), 10);
}
