#!/bin/bash

# Color codes for better output readability
GREEN="\033[1;32m"
RED="\033[1;31m"
YELLOW="\033[1;33m"
CYAN="\033[1;36m"
RESET="\033[0m"

# Check if a day argument is provided
if [ -z "$1" ]; then
    echo -e "${RED}Usage: $0 <day> [part]${RESET}"
    exit 1
fi

# Extract day and part from arguments
DAY="Day$1"
PART=$2

# Base directory of the script
BASE_DIR=$(dirname "$0")

# Navigate to the specified day
DAY_DIR="$BASE_DIR/$DAY"
if [ ! -d "$DAY_DIR" ]; then
    echo -e "${RED}Error: Directory '$DAY_DIR' does not exist.${RESET}"
    exit 1
fi

# Function to compile and run a solution
run_solution() {
    local PART_DIR=$1
    echo -e "${CYAN}-----------------------------------${RESET}"
    echo -e "${CYAN}Running solution in '$PART_DIR'...${RESET}"

    # Check if main.rs and input.txt exist
    if [ ! -f "$PART_DIR/main.rs" ]; then
        echo -e "${RED}Error: main.rs not found in '$PART_DIR'.${RESET}"
        return
    fi
    if [ ! -f "$PART_DIR/input.txt" ]; then
        echo -e "${RED}Error: input.txt not found in '$PART_DIR'.${RESET}"
        return
    fi

    # Compile the Rust solution
    echo -e "${YELLOW}Compiling main.rs...${RESET}"
    rustc "$PART_DIR/main.rs" -o "$PART_DIR/solution"
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: Compilation failed for '$PART_DIR/main.rs'.${RESET}"
        return
    fi

    # Change to the part directory to ensure input.txt is found
    pushd "$PART_DIR" > /dev/null

    # Run the compiled solution and capture its output
    echo -e "${YELLOW}Executing solution...${RESET}"
    RESULT=$(./solution 2>&1)
    if [ $? -ne 0 ]; then
        echo -e "${RED}Error: Execution failed for '$PART_DIR'.${RESET}"
        echo -e "${RED}Details:${RESET}\n$RESULT"
    else
        echo -e "${GREEN}Result for '$PART_DIR':${RESET}"
        echo "$RESULT"
    fi

    # Return to the previous directory
    popd > /dev/null
    echo -e "${CYAN}-----------------------------------${RESET}"
}

# Run all parts for the day if no part is specified
if [ -z "$PART" ]; then
    echo -e "${GREEN}Running all parts for $DAY:${RESET}"
    for PART_DIR in "$DAY_DIR"/Part*; do
        if [ -d "$PART_DIR" ]; then
            run_solution "$PART_DIR"
        fi
    done
else
    # Run the specified part
    PART_DIR="$DAY_DIR/Part$PART"
    if [ -d "$PART_DIR" ]; then
        echo -e "${GREEN}Running Part $PART for $DAY:${RESET}"
        run_solution "$PART_DIR"
    else
        echo -e "${RED}Error: Directory '$PART_DIR' does not exist.${RESET}"
    fi
fi
