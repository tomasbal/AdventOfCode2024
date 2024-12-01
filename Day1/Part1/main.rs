use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    input.trim().lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_list.push(left.parse::<i64>().expect("Invalid number - left list"));
            right_list.push(right.parse::<i64>().expect("Invalid number - right list"));
        }
    });

    left_list.sort_unstable();
    right_list.sort_unstable();

    let total_distance: i64 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Result: {}", total_distance);
}
