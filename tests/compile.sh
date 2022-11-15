#!/bin/bash

# Name of the folders that contain an executable test
executable_tests=("bubblesort")

# Create folder for compiled tests
if [[ ! -d ./compiled ]]; then
    mkdir compiled
fi

echo "Compiling tests..."
# Run through each test and compile it
for test in ${executable_tests[@]}; do
    cd $test
    make
    echo "[*] $test"
    # Remove object files
    make clean_obj
    cd ..
done