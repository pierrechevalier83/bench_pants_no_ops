# Bench pants no-ops

A set of benchmarks tracking pants performance in various situations which should be best case
scenarios for a build system:

These clone a minimal repository with a single source file and measure the time it takes to perform
various operations. Some operations don't require building a build graph (e.g ./pants --version).
Some build a build graph, but there is only one directory and one file, so the cost should be
minimal. Some use v1 or v2, some use a pex or don't use a pex etc.

The idea is to isolate the various sources of fixed overheads where pants users pay for work they
don't use. This should help track work aiming to remove the sources of these overheads

## Status: very early WIP

Note that this is only a very early starting point. The objective here would be to get a
comprehensive list of bottlenecks with usecases isolating these different bottlenecks so we are able
to observe the consequence of fixing each of these bottlenecks.

## Example output on a 2019 Macbook in the current state

```
Benchmarking version: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 1273.4s.
version                 time:   [1.7562 s 1.7974 s 1.8760 s]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) high mild
  1 (10.00%) high severe

Benchmarking version (pex): Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 239.3s.
version (pex)           time:   [2.6007 s 2.6152 s 2.6463 s]
Found 2 outliers among 10 measurements (20.00%)
  1 (10.00%) low mild
  1 (10.00%) high severe

Benchmarking goals: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 97.1s.
goals                   time:   [1.7377 s 1.7627 s 1.8053 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high severe

Benchmarking help: Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 96.5s.
help                    time:   [1.7354 s 1.7482 s 1.7722 s]

Benchmarking binary (v2): Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 97.6s.
binary (v2)             time:   [1.7234 s 1.7501 s 1.8047 s]
Found 1 outliers among 10 measurements (10.00%)
  1 (10.00%) high mild

Benchmarking binary (v1): Warming up for 3.0000 s
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 166.1s.
binary (v1)             time:   [2.6665 s 2.7192 s 2.7792 s]
```
