#!/usr/bin/bash
cd "$(dirname "${BASH_SOURCE[0]}")"

mapfile -t lines < <(cargo run --bin static_str --quiet --release)

if [ ${lines[0]} != ${lines[1]} ]
then
    echo ${lines[0]}
    echo ${lines[1]}
    echo The static/const/literals should have been optimized to have same addresses.
    exit 1
fi
