#!/bin/bash

mkdir Clarfoghlama

cargo build --release

cp ./resources/db.sqlite ./Clarfoghlama
cp ./resources/Rocket.toml ./Clarfoghlama
cp ./resources/libsqlite3.so.0 ./Clarfoghlama/

cp -r ./templates ./Clarfoghlama
cp -r ./static ./Clarfoghlama

cp ./target/release/clarfoghlama ./Clarfoghlama
