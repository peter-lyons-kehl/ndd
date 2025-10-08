#!/bin/sh
set -e
cd "$(dirname "$0")"

cargo run --bin static_option_u8 --quiet --release | {
    read -r line1
    read -r line2
    if [ "$line1" = "$line2" ]; then
        echo "$line1"
        echo "$line2"
        echo "The static/const/literals should NOT have been optimized to have same addresses."
        exit 1
    fi
}
