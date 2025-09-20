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
versions (`gcut` and `gawk`) included as well. [uutils/coreutils](https://github.com/uutils/coreutils) is included as well (labeled `ucut`).

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
1k      gcut        4.565  ms  (4.521  ms  ..  4.604  ms)
1k      choose      4.855  ms  (4.783  ms  ..  4.924  ms)
1k      cut         5.095  ms  (4.986  ms  ..  5.222  ms)
1k      awk         5.889  ms  (5.825  ms  ..  5.925  ms)
1k      ucut        6.213  ms  (6.054  ms  ..  6.483  ms)
1k      gawk        8.761  ms  (8.677  ms  ..  8.862  ms)
10k     choose      5.828  ms  (5.752  ms  ..  5.923  ms)
10k     gcut        6.144  ms  (6.019  ms  ..  6.337  ms)
10k     ucut        6.839  ms  (6.788  ms  ..  6.901  ms)
10k     gawk        12.80  ms  (12.44  ms  ..  13.04  ms)
10k     cut         14.08  ms  (14.01  ms  ..  14.16  ms)
10k     awk         22.48  ms  (22.26  ms  ..  22.87  ms)
100k    ucut        13.67  ms  (13.49  ms  ..  13.85  ms)
100k    choose      15.23  ms  (15.06  ms  ..  15.36  ms)
100k    gcut        21.66  ms  (21.43  ms  ..  21.95  ms)
100k    gawk        56.48  ms  (56.10  ms  ..  57.11  ms)
100k    cut         102.1  ms  (101.6  ms  ..  102.6  ms)
100k    awk         186.0  ms  (185.8  ms  ..  186.6  ms)
1000k   ucut        78.37  ms  (77.51  ms  ..  78.97  ms)
1000k   choose      107.5  ms  (103.8  ms  ..  110.8  ms)
1000k   gcut        173.4  ms  (171.9  ms  ..  174.8  ms)
1000k   gawk        488.1  ms  (NaN    s   ..  495.9  ms)
1000k   cut         980.0  ms  (977.9  ms  ..  982.4  ms)
1000k   awk         1.831  s   (NaN    s   ..  1.858  s)
10000k  ucut        723.9  ms  (700.7  ms  ..  757.3  ms)
10000k  choose      1.045  s   (1.022  s   ..  1.067  s)
10000k  gcut        1.672  s   (1.662  s   ..  1.684  s)
10000k  gawk        4.739  s   (4.660  s   ..  4.783  s)
10000k  cut         9.761  s   (9.721  s   ..  9.783  s)
10000k  awk         18.17  s   (17.96  s   ..  18.36  s)
```
