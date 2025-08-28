#!/bin/bash
cargo build --release # always be up to date
output="bench_output"
mkdir -p $output
inputs="$(find test -name "long*txt")"
for i in $inputs; do
  echo ${i}
  bench "target/release/choose 3:5 -i ${i}" > $output/choose_$(basename $i .txt).bench
  bench "cut -f 4-6 -d ' ' ${i}" > $output/cut_$(basename $i .txt).bench
  bench "awk '{print $4 $5 $6}' ${i}" > $output/awk_$(basename $i .txt).bench
done

grep time $output/* | sort -r | column -t
