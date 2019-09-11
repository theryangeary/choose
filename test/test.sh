#!/bin/bash
set -e

test_dir="test"
orig_dir="$(pwd)"
cd "$(git rev-parse --show-toplevel)"
cargo build

diff -Z <(cargo run -- 0:2 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_0:2.txt")
diff -Z <(cargo run -- 0 3 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_0_3.txt")
diff -Z <(cargo run -- :2 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_:2.txt")
diff -Z <(cargo run -- 9 3 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_9_3.txt")
diff -Z <(cargo run -- 9 -i ${test_dir}/lorem.txt) <(cat "${test_dir}/choose_9.txt")
