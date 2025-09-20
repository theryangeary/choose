#!/bin/bash
cargo build --release # always be up to date
output="bench_output"
mkdir -p $output
inputs="$(find test -name "benchinput*txt")"
for i in $inputs; do
  echo ${i}
  bench "target/release/choose 3:5 -i ${i}" > $output/choose$(basename $i .txt).bench
  bench "coreutils cut -f 4-6 -d ' ' ${i}" > $output/ucut$(basename $i .txt).bench
  bench "cut -f 4-6 -d ' ' ${i}" > $output/cut$(basename $i .txt).bench
  bench "gcut -f 4-6 -d ' ' ${i}" > $output/gcut$(basename $i .txt).bench
  bench "awk '{print \$4, \$5, \$6}' ${i}" > $output/awk$(basename $i .txt).bench
  bench "gawk '{print \$4, \$5, \$6}' ${i}" > $output/gawk$(basename $i .txt).bench
done

grep time $output/* | awk '{ value=$2; unitvalue=$3; };
    / s/ { m=1000 }; / ms/ { m=1 };
    { sub(" s","",unitvalue); value=value*m;
    print value " " $0; }' | choose -f '(benchinput| )' 2 0 1 3: | sort -V | choose -f '(\.b|/)' 0 2: | column -t
#                                                       ^^^^^^^ try doing this with cut
