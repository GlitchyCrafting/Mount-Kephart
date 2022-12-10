mkdir Clarfoghlama

cargo build --release

cp ./resources/db.sqlite ./Clarfoghlama
cp ./resources/Rocket.toml ./Clarfoghlama
cp ./resources/sqlite3.dll ./Clarfoghlama/

cp -r ./templates ./Clarfoghlama
cp -r ./static ./Clarfoghlama

cp ./target/release/clarfoghlama.exe ./Clarfoghlama
