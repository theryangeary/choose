flamegraph: release
	perf record --call-graph dwarf,16384 -e cpu-clock -F 997 target/release/choose -i test/long_long_long_long.txt 3:5
	perf script | stackcollapse-perf.pl | stackcollapse-recursive.pl | c++filt | flamegraph.pl > flamegraphs/working.svg

flamegraph_commit: release
	perf record --call-graph dwarf,16384 -e cpu-clock -F 997 target/release/choose -i test/long_long_long_long.txt 3:5
	perf script | stackcollapse-perf.pl | stackcollapse-recursive.pl | c++filt | flamegraph.pl > flamegraphs/`git log -n 1 --pretty=format:"%h"`.svg

.PHONY: test
test:
	test/e2e_test.sh

bench: release
	test/bench.sh working

bench_commit: release
	test/bench.sh `git log -n 1 --pretty=format:"%h"`

.PHONY: release
release:
	cargo build --release
