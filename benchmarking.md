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
you want to test. The default configuration here is for a machine where the
default `cut` and `awk` are BSD versions, therefore there are additional GNU
versions (`gcut` and `gawk`) included as well.

You want to make sure that the output of all of your `bench` commands is the
same, otherwise comparing the performance will be meaningless.

Once your `bench` commands are set, just run the script:

```bash
test/bench.sh
```

and go grab a coffee, especially if you're including BSD awk.

## Sample Results

The results from a 2021 MacBook Pro w/ M1 processor and 16GB of RAM are below.

The first column is the input file suffix (whatever comes after `benchinput`).

```
1k      gcut    4.515  ms  (4.502  ms  ..  4.530  ms)
1k      choose  4.869  ms  (4.824  ms  ..  4.919  ms)
1k      cut     5.129  ms  (5.066  ms  ..  5.201  ms)
1k      awk     5.953  ms  (5.885  ms  ..  6.060  ms)
1k      gawk    8.629  ms  (8.570  ms  ..  8.681  ms)
10k     gcut    6.039  ms  (5.970  ms  ..  6.079  ms)
10k     choose  6.308  ms  (6.235  ms  ..  6.413  ms)
10k     gawk    12.86  ms  (12.67  ms  ..  12.98  ms)
10k     cut     13.92  ms  (13.89  ms  ..  13.95  ms)
10k     awk     22.24  ms  (22.22  ms  ..  22.28  ms)
100k    choose  20.59  ms  (20.40  ms  ..  20.90  ms)
100k    gcut    21.43  ms  (21.19  ms  ..  21.69  ms)
100k    gawk    56.12  ms  (55.86  ms  ..  56.66  ms)
100k    cut     101.4  ms  (101.2  ms  ..  101.7  ms)
100k    awk     186.9  ms  (180.2  ms  ..  197.4  ms)
1000k   choose  162.8  ms  (160.1  ms  ..  166.1  ms)
1000k   gcut    171.8  ms  (171.0  ms  ..  173.7  ms)
1000k   gawk    483.9  ms  (482.5  ms  ..  485.9  ms)
1000k   cut     979.9  ms  (976.7  ms  ..  985.9  ms)
1000k   awk     1.825  s   (1.811  s   ..  1.848  s)
10000k  choose  1.555  s   (1.528  s   ..  1.577  s)
10000k  gcut    1.683  s   (1.638  s   ..  1.712  s)
10000k  gawk    4.740  s   (4.655  s   ..  4.806  s)
10000k  cut     9.765  s   (9.653  s   ..  9.865  s)
10000k  awk     18.16  s   (18.03  s   ..  18.39  s)
```
