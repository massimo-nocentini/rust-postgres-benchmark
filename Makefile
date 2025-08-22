
build:
	cargo build --release

tweet_fetch:	
	DATABASE_URL="postgres://postgres:password@localhost/benchmark_db" cargo run --release --bin tweet_fetch 

threaded:
	DATABASE_URL="postgres://postgres:password@localhost/benchmark_db" cargo run --release --bin threaded -- 10000 ./postgres/simple-insert.sql

dockerup:
	docker compose up
