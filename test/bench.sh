#!/bin/bash
cargo build --release # always be up to date
output="bench_output"
mkdir -p $output
inputs=($(find test -name "long*txt" | sort -r))
for i in {0..4}
do
  echo ${inputs[$i]}
  bench "target/release/choose 3:5 -i ${inputs[$i]}"  > $output/$1$i.bench
done
