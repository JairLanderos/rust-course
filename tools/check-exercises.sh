#!/usr/bin/env bash
# Checks every exercise in exercises/:
#   - its starting code fails, as designed (doesn't compile, or fails its tests);
#   - with its solution from solutions/ copied over, it passes its tests with no warnings.
# Each exercise is checked as a standalone package in a temporary directory.
set -uo pipefail

root=$(cd "$(dirname "$0")/.." && pwd)
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
export CARGO_TARGET_DIR="$tmp/target"
status=0

for template in "$root"/exercises/*/*/; do
    name=$(basename "$template")
    section=$(basename "$(dirname "$template")")
    solution="$root/solutions/$section/$name"
    work="$tmp/$name"

    cp -r "$template" "$work"
    cp "$root/rust-toolchain.toml" "$work/"
    rm -rf "$work/target" "$work/Cargo.lock"

    if (cd "$work" && cargo test --quiet >/dev/null 2>&1); then
        echo "FAIL  $name: the starting code already passes its tests"
        status=1
        continue
    fi
    if [ ! -d "$solution" ]; then
        echo "FAIL  $name: no solution in solutions/$section/$name"
        status=1
        continue
    fi

    cp -r "$solution"/. "$work"/
    if (cd "$work" && RUSTFLAGS="-D warnings" cargo test --quiet >"$tmp/$name.log" 2>&1); then
        echo "ok    $name"
    else
        echo "FAIL  $name: the solution doesn't pass"
        sed 's/^/      /' "$tmp/$name.log"
        status=1
    fi
done

exit $status
