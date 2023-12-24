#!/bin/bash
# Extract the problem name from the path
problem_name=$(echo $1 | sed 's/\/src//') # Removes the '/src' part
mold -run cargo run --bin "$problem_name"