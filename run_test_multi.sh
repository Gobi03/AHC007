#/bin/bash

set -eu

cargo build --release

test_num="0000"
../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"

cd ..
test_num="0001"
../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"

cd ..
test_num="0002"
../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"

cd ..
test_num="0003"
../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"

cd ..
test_num="0004"
../target/release/a < "tools/in/${test_num}.txt" > "tools/out/${test_num}.txt"
cd tools
cargo run --release --bin vis "in/${test_num}.txt" "out/${test_num}.txt"