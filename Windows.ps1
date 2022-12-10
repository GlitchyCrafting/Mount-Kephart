mkdir Mount\ Kephart

cargo build --release

cp ./resources/db.sqlite ./Mount\ Kephart
cp ./resources/Rocket.toml ./Mount\ Kephart
cp ./resources/sqlite3.dll ./Mount\ Kephart/

cp -r ./templates ./Mount\ Kephart
cp -r ./static ./Mount\ Kephart

cp ./target/release/mount_kephart.exe ./Mount\ Kephart
