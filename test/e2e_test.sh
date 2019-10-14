#!/bin/bash
set -e

test_dir="test"
orig_dir="$(pwd)"
cd "$(git rev-parse --show-toplevel)"
cargo build

# basic functionality
diff -w <(cargo run -- 0:2 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_0:2.txt")
diff -w <(cargo run -- 0 3 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_0_3.txt")
diff -w <(cargo run -- :2 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_:2.txt")
diff -w <(cargo run -- 9 3 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_9_3.txt")
diff -w <(cargo run -- 9 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_9.txt")
diff -w <(cargo run -- 12 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_12.txt")
diff -w <(cargo run -- 4:1 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_4:1.txt")
# add tests for different delimiters
# add tests using piping

set +e

# test failure to parse arguments
cargo run -- d:i -i ${test_dir}/lorem.txt >/dev/null
r=$?
if [ $r -ne 2 ]; then
  echo "Failed to return error code 2 on failure to parse arguments"
else
  echo "Success"
fi

cargo test

cd $orig_dir
