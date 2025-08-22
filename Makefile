
build:
	cargo build --release

run:	
	DATABASE_URL="postgres://postgres:password@localhost/benchmark_db" cargo run tweet_fetch --release

docker-up:
	docker compose up
