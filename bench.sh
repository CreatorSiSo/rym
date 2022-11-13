cargo build &&
	cargo build -r &&
	cargo build --profile dist

cp ./target/debug/rymx ./bench/debug_rymx
cp ./target/release/rymx ./bench/release_rymx
cp ./target/dist/rymx ./bench/dist_rymx
