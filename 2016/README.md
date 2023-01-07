# 2016

Solutions are in the `examples` folder. For day `xx` from `01` to `25`, there
are 3 files:
- `xx-1.rs` for part 1
- `xx-2.rs` for part 2 (except for `25-2.rs`)
- `xx.txt` for the input

The command line to run the solution for day `xx` part `y` is:

```shell
cargo run --release --example=xx-y
```

The command line to test a solution is:

```shell
cargo test --release --example=xx-y
```

The `test.sh` script can be used to check formatting, linting, and that all
solutions still pass their test.
