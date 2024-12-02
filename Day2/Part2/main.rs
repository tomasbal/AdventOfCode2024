use std::fs;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    let reports_str = contents.split_terminator('\n');
    let reports: Vec<Vec<u8>> = reports_str
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse().expect("Invalid level"))
                .collect()
        })
        .collect();

    let num_safe_reports = reports
        .iter()
        .filter(|report| {
            is_safe(report) || generate_subreports(report).any(|subreport| is_safe(&subreport))
        })
        .count();

    println!("Result: {}", num_safe_reports);
}

fn is_safe(report: &Vec<u8>) -> bool {
    let is_sorted_by = |report: &Vec<u8>, cmp: fn(u8, u8) -> bool| -> bool {
        report.windows(2).all(|pair| cmp(pair[0], pair[1]))
    };

    let is_increasing = |report: &Vec<u8>| is_sorted_by(report, |x, y| x < y);
    let is_decreasing = |report: &Vec<u8>| is_sorted_by(report, |x, y| x > y);
    let is_in_range = |report: &Vec<u8>| {
        report
            .windows(2)
            .map(|s| (s[0], s[1]))
            .all(|(x, y)| x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3)
    };

    (is_increasing(report) || is_decreasing(report)) && is_in_range(report)
}

/// Generates all possible subreports by removing one element at each position in the input.
fn generate_subreports(report: &Vec<u8>) -> impl Iterator<Item = Vec<u8>> + '_ {
    (0..report.len()).map(move |index| {
        report
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != index)
            .map(|(_, &val)| val)
            .collect()
    })
}