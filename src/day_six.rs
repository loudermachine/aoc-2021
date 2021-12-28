#[derive(Clone, Copy, Debug)]
struct Lanternfish {
    pub lifetime: i32
}

impl Lanternfish {
    pub fn new(lifetime: i32) -> Self {
        Self { lifetime }
    }
}


impl From<&str> for Lanternfish {
    fn from(s: &str) -> Self {
        Lanternfish {
            lifetime: s.parse::<i32>().unwrap()
        }
    }
}

struct ShoalNaive {
    fishes: Vec<Lanternfish>
}

impl ShoalNaive {
    pub fn simulate(&mut self, days: usize) -> usize {
        for _ in 0..days {
            let mut idx = 0;
            let curr_size = self.fishes.len();
            loop {
                if idx == curr_size {
                    break
                }

                let mut fish = &mut self.fishes[idx];
                if fish.lifetime == 0 {
                    fish.lifetime = 6;
                    self.fishes.push(Lanternfish::new(8));
                } else {
                    fish.lifetime -= 1;
                }

                idx += 1
            }
        }

        self.fishes.len()
    }
}

impl From<&str> for ShoalNaive {
    fn from(s: &str) -> Self {
        let fishes = s.split(',').map(Lanternfish::from).collect::<Vec<Lanternfish>>();
        Self { fishes }
    }
}

struct Shoal {
    bucket: Vec<usize>
}

impl Shoal {
    pub fn simulate(mut self, days: usize) -> usize {
        for _ in 0..days {
            self.bucket.rotate_left(1);
            self.bucket[6] += self.bucket[8];
        }

        self.bucket.iter().sum()
    }
}

impl From<&str> for Shoal {
    fn from(s: &str) -> Self {
        let mut bucket = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        s.split(',').for_each(|n| {
            let lifetime = n.parse::<usize>().unwrap();
            bucket[lifetime] += 1;
        });

        Self { bucket }
    }
}

#[test]
fn day_six_part_one() {
    let input = std::fs::read_to_string("./data/day_six.txt").unwrap();
    let mut shoal = ShoalNaive::from(input.as_str());
    let result = shoal.simulate(80);
    println!("How many lanternfish would there be after 80 days? {}", result);
}

#[test]
fn day_six_part_two() {
    let input = std::fs::read_to_string("./data/day_six.txt").unwrap();
    let shoal = Shoal::from(input.as_str());
    let result = shoal.simulate(256);
    println!("How many lanternfish would there be after 256 days? {}", result);
}

#[test]
fn day_six_part_one_example() {
    let input = r#"3,4,3,1,2"#;
    let mut shoal = ShoalNaive::from(input);
    assert_eq!(5934, shoal.simulate(80));
}

#[test]
fn day_size_part_two_example() {
    let input = r#"3,4,3,1,2"#;
    let shoal = Shoal::from(input);
    assert_eq!(26984457539, shoal.simulate(256));
}