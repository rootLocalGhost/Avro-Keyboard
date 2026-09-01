#!/bin/bash

echo "Running tests..."
CLI="./src/core/avro_cli"

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
  echo "Some tests failed!"
  exit 1
else
  echo "All tests passed!"
fi
