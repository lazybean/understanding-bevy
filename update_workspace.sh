#!/bin/bash

echo "[workspace]" > Cargo.toml
echo "members = [" >> Cargo.toml
find src -name Cargo.toml | xargs dirname | xargs -I % echo '  "%",' >> Cargo.toml
echo "]" >> Cargo.toml
