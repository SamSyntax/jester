build:
	cargo build --release --bin parser

run-lg:
	make build && ./target/release/parser ./large-file.json

run-md:
	make build && ./target/release/parser ./large-file.json

run:
	make build && ./target/release/parser 
