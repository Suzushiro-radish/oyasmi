#!/bin/bash
assert() {
    expected="$1"
    input="$2"

    ./target/debug/oyasmi "$input" >tmp.wat || exit
    actual=$(wasmtime run tmp.wat || exit)

    if [ "$actual" = "$expected" ]; then
        echo "$input => $actual"
    else
        echo "$input => $expected expected, but got $actual"
        exit 1
    fi
}

cargo build

assert 0 0
assert 42 42
assert 15 "5+10"
assert 8 "17-9"
assert 21 "5+20-4"
assert 21 " 5 +  20 - 4 "
assert 12 "3 * 4"
assert 3 "15 / 4"
assert 9 "1 + 2 * 4"
assert 5 "2 / 2 + 4"
assert 12 "2 * (2 + 4)"
assert 4 "2 * (2 + 4) / (5 - (1 + 1))"
assert 1 "2; 1;"
assert 5 "1 * 8; 20/4"

echo OK
