use std::fs;

fn main() {
    // Read input file
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read file");

    // Parse the input into reports
    let reports_str = contents.split_terminator('\n');
    let reports: Vec<Vec<u8>> = reports_str
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| level.parse().expect("Invalid level"))
                .collect()
        })
        .collect();

    // Determine if reports are safe
    let num_safe_reports = reports.iter().filter(|&report| is_safe(report)).count();
    println!("Result: {}", num_safe_reports);
}

/// Checks if a report is safe by verifying sorting and range constraints.
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
