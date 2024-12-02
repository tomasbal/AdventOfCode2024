# Advent of Code 2024


This repository contains solutions for Advent of Code 2024 challenges implemented in Rust. Each day is organized into separate directories, with subdirectories for individual parts of the challenge.

---

## Project Structure
```markdown

root/
├── Day1/
│   ├── Part1/
│   │   ├── main.rs        # Rust solution for Part 1
│   │   ├── input.txt      # Input file for Part 1
│   ├── Part2/
│   │   ├── main.rs        # Rust solution for Part 2
│   │   ├── input.txt      # Input file for Part 2
├── Day2/
│   ├── Part1/
│   ├── Part2/
├── run.sh                 # Bash script to compile and run solutions
├── .gitignore             # Git ignore rules
└── README.md              # Project documentation

```

## How to Use

### Prerequisites

- **Rust**: Ensure you have [Rust](https://www.rust-lang.org/) installed.
- **Bash Shell**: The `run.sh` script requires a Unix-like environment (Linux, macOS, or WSL on Windows).

### Running Solutions

#### Run All Parts for a Day

To run all parts for a specific day, use the `run.sh` script:

```bash
./run.sh <day>
```

Example:

```bash
./run_solutions.sh 1
```

#### Run a Specific Part

To run a specific part of a day:

```bash
./run_solutions.sh <day> <part>
```

Example:

```bash
./run_solutions.sh 1 2
```

---

## Input Files

- Each part expects an `input.txt` file in the same directory as its `main.rs`.
- Ensure the `input.txt` file is present with valid data for the solution to execute correctly.

---

## Adding a New Day

1. Create a directory for the day:
   ```bash
   mkdir -p Day<day>/Part<part>
   ```

2. Add `main.rs` and `input.txt` files:
    - `main.rs`: Write your Rust solution.
    - `input.txt`: Add the input data.

3. Run the solution using the `run_solutions.sh` script.

---

## Example Output

### Running Day 1 Solutions

```bash
$ ./run_solutions.sh 1
-----------------------------------
Running solution in './Day1/Part1'...
Compiling main.rs...
Executing solution...
Result for './Day1/Part1':
42
-----------------------------------
Running solution in './Day1/Part2'...
Compiling main.rs...
Executing solution...
Result for './Day1/Part2':
1337
-----------------------------------
```
