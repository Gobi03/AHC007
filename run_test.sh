#/bin/bash

set -eu

test_num="0000"

cargo build --release
cargo run --release --bin a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"

cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"
