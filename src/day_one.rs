use std::fs;

fn read_measures() -> Vec<i32> {
    let input = fs::read_to_string("./data/day_one.txt").unwrap();
    input
        .split("\n")
        .map(|m| m.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn part_one() {
    let measures = read_measures();
    let mut measures = measures.iter();
    let mut last_measure = measures.next().unwrap();
    let mut count = 0;
    for measure in measures {
        if measure > last_measure {
            count += 1
        }
        last_measure = measure
    }

    dbg!(count);
}

#[test]
fn part_two() {
    let measures = read_measures();
    let (mut slice_start, mut slice_end) = (0, 3);
    let mut last_sum: i32 = measures[slice_start..slice_end].iter().sum();
    let mut count = 0;
    loop {
        slice_start += 1;
        slice_end += 1;

        if slice_end > measures.len() {
            break;
        }

        let curr_sum: i32 = measures[slice_start..slice_end].iter().sum();
        if curr_sum > last_sum {
            count += 1;
        }
        last_sum = curr_sum;
    }

    println!(
        "How many sums are larger than the previous sum?\nR: {}",
        count
    )
}
