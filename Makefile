
build:
	cargo build --release

run:	
	DATABASE_URL="postgres://postgres:password@localhost/benchmark_db" cargo run --release

docker-up:
	docker compose up
