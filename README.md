Reproducer for a priority inversion deadlock with std::sync::mpsc::channel try_recv.

## Instructions

Inside repository directory on Linux:

1. cargo build # optionally add "--features crossbeam"
2. sudo ./target/debug/mpsc_deadlock_reproducer

https://github.com/rust-lang/rust/issues/112723
https://github.com/crossbeam-rs/crossbeam/issues/997
