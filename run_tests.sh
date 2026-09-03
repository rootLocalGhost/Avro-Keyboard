#!/bin/bash

echo "Running cargo tests..."
cargo test --workspace
if [ $? -ne 0 ]; then
  echo "Cargo tests failed!"
  exit 1
fi

echo "Building avro-cli..."
cargo build -p avro-cli
CLI="./target/debug/avro-cli"

echo "Running integration tests..."
FAILS=0

check() {
  input=$1
  expected=$2
  result=$($CLI "$input")
  if [ "$result" = "$expected" ]; then
    echo "PASS: $input -> $result"
  else
    echo "FAIL: $input -> $result (Expected: $expected)"
    FAILS=1
  fi
}

check "amra" "আম্রা"
check "bangla" "বাংলা"
check "kotha" "কথা"
check "shikkha" "শিক্ষা"

if [ $FAILS -ne 0 ]; then
  echo "Some integration tests failed!"
  exit 1
else
  echo "All tests passed!"
fi
