# Benchmarking

## Disclaimer

Benchmarking results will vary with workload (i.e. arguments to `choose` and
content of file), hardware, OS, and other factors. Results presented here are
just one possible combination of these factors, and should be treated not as
any guarantee or promise.

If you care about the performance of `choose` consider running your own
workload in your own environment (and by all means share the results).

## Benchmarking Prerequisites

Benchmarking is performed using the [`bench` utility](https://github.com/Gabriel439/bench).

Benchmarking is based on the assumption that there are some files in `test/`
that match the glob "benchinput*txt". GitHub doesn't support files big enough in
normal repos, but for reference the files I'm working with have lengths like
these:

```sh
    1000 test/benchinput1k.txt
   10000 test/benchinput10k.txt
  100000 test/benchinput100k.txt
 1000000 test/benchinput1000k.txt
10000000 test/benchinput10000k.txt
```

and content from [lorem.txt](test/lorem.txt).

## Running Benchmarking

First, customize the `bench` commands in `test/bench.sh` to run the workload
you want to test. The default configuration here is for a BSD (macOS) machine,
where the default `cut` and `awk` are BSD versions, therefore there are
additional GNU versions included as well.

You want to make sure that the output of all of your `bench` commands is the
same, otherwise comparing the performance will be meaningless.

Once your `bench` commands are set, just run the script:

```bash
test/bench.sh
```

and go grab a coffee, especially if you're including BSD awk.

## Sample Results

On my 2021 MacBook Pro w/ M1 processor and 16GB of RAM, my results are:

```
1k      choose  5.103  ms  (5.082  ms  ..  5.126  ms)
1k      cut     5.362  ms  (5.289  ms  ..  5.469  ms)
1k      gcut    5.603  ms  (4.867  ms  ..  6.921  ms)
1k      awk     6.070  ms  (5.994  ms  ..  6.177  ms)
1k      gawk    9.074  ms  (9.018  ms  ..  9.125  ms)
10k     gcut    6.095  ms  (6.028  ms  ..  6.222  ms)
10k     choose  6.971  ms  (6.454  ms  ..  7.490  ms)
10k     gawk    10.16  ms  (10.06  ms  ..  10.31  ms)
10k     cut     14.00  ms  (13.93  ms  ..  14.07  ms)
10k     awk     20.93  ms  (20.84  ms  ..  21.01  ms)
100k    choose  21.85  ms  (21.65  ms  ..  22.08  ms)
100k    gcut    22.52  ms  (22.41  ms  ..  22.68  ms)
100k    gawk    28.34  ms  (27.85  ms  ..  28.88  ms)
100k    cut     105.0  ms  (104.8  ms  ..  105.1  ms)
100k    awk     176.7  ms  (176.6  ms  ..  176.8  ms)
1000k   choose  164.8  ms  (156.7  ms  ..  167.5  ms)
1000k   gcut    176.0  ms  (174.4  ms  ..  177.1  ms)
1000k   gawk    195.5  ms  (195.0  ms  ..  195.9  ms)
1000k   cut     1.012  s   (1.004  s   ..  1.022  s)
1000k   awk     1.722  s   (1.689  s   ..  1.743  s)
10000k  choose  1.560  s   (1.552  s   ..  1.567  s)
10000k  gcut    1.682  s   (1.658  s   ..  1.713  s)
10000k  gawk    1.850  s   (1.759  s   ..  1.898  s)
10000k  cut     9.748  s   (9.672  s   ..  9.847  s)
10000k  awk     16.89  s   (16.67  s   ..  NaN    s)
```

The first column is the input file suffix (whatever comes after `benchinput`).
