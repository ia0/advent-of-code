# 2022

The command line to run a solution is:

```shell
cargo run --example=<day>-<part>
```

Where `<day>` is the 2-digit day and `<part>` is 1 or 2. For example, the
command line to run the solution for day 14 part 2 is:

```shell
cargo run --example=14-2
```

The command line to test a solution is:

```shell
cargo test --example=14-2
```

The `test.sh` script can be used to check formatting, linting, and that all
solutions still pass their test.
