#!/bin/sh

# Chaos v1 Test Runner

set -u

ROOT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
TEST_DIR="$ROOT_DIR/tests"
CHAOS="$ROOT_DIR/chaos"

PASSED=0
FAILED=0
TOTAL=0

printf '\n'
printf '%s\n' '========================================'
printf '%s\n' '        CHAOS v1 TEST SUITE'
printf '%s\n' '========================================'
printf '\n'

cd "$ROOT_DIR" || exit 1

printf '%s\n' 'Building Chaos...'
printf '\n'

if ! make; then
    printf '\n%s\n' 'Build failed. Tests cannot run.'
    exit 1
fi

printf '\n'
printf '%s\n' 'Build successful.'
printf '\n'

if [ ! -x "$CHAOS" ]; then
    printf '%s\n' "Error: Chaos executable not found: $CHAOS"
    exit 1
fi

if [ ! -d "$TEST_DIR" ]; then
    printf '%s\n' "Error: test directory not found: $TEST_DIR"
    exit 1
fi

printf '%s\n' 'Running tests...'
printf '\n'

TEST_LIST="$(find "$TEST_DIR" -type f -name '*.chaos' | sort)"

if [ -z "$TEST_LIST" ]; then
    printf '%s\n' 'No Chaos test files found.'
    exit 1
fi

OLD_IFS="$IFS"
IFS='
'

for test_file in $TEST_LIST; do

    TOTAL=$((TOTAL + 1))

    relative_path="${test_file#$ROOT_DIR/}"

    printf '%s\n' '----------------------------------------'
    printf '%s\n' "TEST: $relative_path"
    printf '%s\n' '----------------------------------------'

    if "$CHAOS" "$test_file"; then
        printf '\n'
        printf '%s\n' "PASS: $relative_path"
        PASSED=$((PASSED + 1))
    else
        printf '\n'
        printf '%s\n' "FAIL: $relative_path"
        FAILED=$((FAILED + 1))
    fi

    printf '\n'
done

IFS="$OLD_IFS"

printf '%s\n' '========================================'
printf '%s\n' '              TEST RESULTS'
printf '%s\n' '========================================'
printf '%s\n' "Total:  $TOTAL"
printf '%s\n' "Passed: $PASSED"
printf '%s\n' "Failed: $FAILED"
printf '%s\n' '========================================'
printf '\n'

if [ "$FAILED" -eq 0 ]; then
    printf '%s\n' 'All Chaos v1 tests passed.'
    exit 0
fi

printf '%s\n' 'Some Chaos v1 tests failed.'
exit 1