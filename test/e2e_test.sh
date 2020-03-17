#!/bin/bash
set -e

test_dir="test"
orig_dir="$(pwd)"
cd "$(git rev-parse --show-toplevel)"

# basic functionality
diff -w <(cargo run -- 0:1 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_0:1.txt")
diff -w <(cargo run -- 0 3 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_0_3.txt")
diff -w <(cargo run -- :1 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_:1.txt")
diff -w <(cargo run -- 9 3 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_9_3.txt")
diff -w <(cargo run -- 9 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_9.txt")
diff -w <(cargo run -- 12 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_12.txt")
diff -w <(cargo run -- 4:2 -i ${test_dir}/lorem.txt 2>/dev/null) <(cat "${test_dir}/choose_4:2.txt")
# add tests for different delimiters
# add tests using piping

set +e

# test failure to parse arguments
cargo run -- d:i -i ${test_dir}/lorem.txt >&/dev/null
r=$?
if [ $r -ne 1 ]; then
  echo "Failed to return 1 on failure to parse arguments"
  exit 1
fi

cargo run -- 3 -f "[[]" -i ${test_dir}/lorem.txt >&/dev/null
r=$?
if [ $r -ne 2 ]; then
  echo "Failed to return 2 on regex compile error"
  exit 1
fi

file=/tmp/000_file
touch $file
chmod 000 $file
cargo run -- 3 -i $file >&/dev/null
r=$?
if [ $r -ne 3 ]; then
  echo "Failed to return 3 on file open error"
  exit 1
fi
rm -f $file

cd $orig_dir

printf "\033[1;32mAll tests passed\033[0m\n"
