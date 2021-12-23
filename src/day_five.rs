use std::{collections::HashMap, mem::swap};

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        match &s.split(',').collect::<Vec<&str>>()[..] {
            [x, y] => Self {
                x: x.parse::<i32>().unwrap(),
                y: y.parse::<i32>().unwrap(),
            },
            _ => unreachable!(),
        }
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        match &s.split_whitespace().collect::<Vec<&str>>()[..] {
            [p1, "->", p2] => Self {
                p1: Point::from(*p1),
                p2: Point::from(*p2),
            },
            _ => unreachable!(),
        }
    }
}

fn map_overlap_straight(map: &mut HashMap<(i32, i32), usize>, x0: i32, x1: i32, y0: i32, y1: i32) {
    let mut x0 = x0;
    let mut x1 = x1;
    let mut y0 = y0;
    let mut y1 = y1;

    if x0 == x1 {
        if y0 > y1 {
            swap(&mut y0, &mut y1)
        }
        let x = x0;
        for y in y0..=y1 {
            map.insert((x, y), *map.get(&(x, y)).or(Some(&0)).unwrap() + 1);
        }
    } else {
        if x0 > x1 {
            swap(&mut x0, &mut x1)
        }
        let y = y0;
        for x in x0..=x1 {
            map.insert((x, y), *map.get(&(x, y)).or(Some(&0)).unwrap() + 1);
        }
    }
}

fn map_overlap_diagonal(map: &mut HashMap<(i32, i32), usize>, x0: i32, x1: i32, y0: i32, y1: i32) {
    let mut x = x0;
    let mut y = y0;
    loop {
        map.insert((x, y), *map.get(&(x, y)).or(Some(&0)).unwrap() + 1);
        if x == x1 && y == y1 {
            break;
        }
        if x > x1 {
            x -= 1
        } else if x < x1 {
            x += 1
        }

        if y > y1 {
            y -= 1
        } else if y < y1 {
            y += 1
        }
    }
}

fn count_overlapping(lines: Vec<Line>) -> u32 {
    let mut map = HashMap::<(i32, i32), usize>::new();
    for line in lines.iter() {
        let (x0, y0, x1, y1) = (line.p1.x, line.p1.y, line.p2.x, line.p2.y);
        map_overlap_diagonal(&mut map, x0, x1, y0, y1);
    }

    let mut count = 0;
    for k in map.keys() {
        let v = map.get(k).unwrap();
        if *v > 1 {
            count += 1;
        }
    }

    count
}

#[test]
fn day_five_part_one() {
    let input = std::fs::read_to_string("./data/day_five.txt").unwrap();
    let lines = input
        .split('\n')
        .map(|l| Line::from(l))
        .filter(|l| l.p1.x == l.p2.x || l.p1.y == l.p2.y)
        .collect();
    let count = count_overlapping(lines);
    println!(
        "At how many points do at least two lines overlap?\nR: {}",
        count
    )
}

#[test]
fn day_five_part_two() {
    let input = std::fs::read_to_string("./data/day_five.txt").unwrap();
    let lines = input.split('\n').map(|l| Line::from(l)).collect();
    let count = count_overlapping(lines);
    println!(
        "At how many points do at least two lines overlap?\nR: {}",
        count
    )
}

#[test]
fn day_five_part_one_example() {
    let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
    let lines = input
        .split('\n')
        .map(|l| Line::from(l))
        .filter(|l| l.p1.x == l.p2.x || l.p1.y == l.p2.y)
        .collect();
    let count = count_overlapping(lines);
    assert_eq!(5, count)
}
