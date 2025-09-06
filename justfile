build :
	cargo build --features surrealdb/kv-rocksdb 
release:
	cargo build --features surrealdb/kv-rocksdb --release
run:
	cargo run --features surrealdb/kv-rocksdb

