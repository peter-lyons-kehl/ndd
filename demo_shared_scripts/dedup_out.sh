#!/bin/sh
# Invoke with two parameters: name_of_the_binary_crate name_of_build_profile
set -e
cd "$(dirname "$0")"

cargo run --bin $1 --quiet --profile $2 | {
    read -r line1
    read -r line2
    if [ "$line1" != "$line2" ]; then
        echo "$line1"
        echo "$line2"
        echo "The static/const/literals should have been optimized to have same addresses."
        exit 1
    fi
}
