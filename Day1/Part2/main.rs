use std::collections::HashMap;
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

    let right_freq: HashMap<i64, usize> = right_list.into_iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let similarity_score: i64 = left_list.iter().fold(0, |score, &num| {
        score + num * right_freq.get(&num).copied().unwrap_or(0) as i64
    });

    println!("Result: {}", similarity_score);
}
