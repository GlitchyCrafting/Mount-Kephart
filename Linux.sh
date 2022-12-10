#!/bin/bash

mkdir Mount\ Kephart

cargo build --release

cp ./resources/db.sqlite ./Mount\ Kephart
cp ./resources/Rocket.toml ./Mount\ Kephart
cp ./resources/libsqlite3.so.0 ./Mount\ Kephart/

cp -r ./templates ./Mount\ Kephart
cp -r ./static ./Mount\ Kephart

cp ./target/release/mount_kephart ./Mount\ Kephart
