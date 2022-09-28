docker:
	docker buildx build . -t novops:local --load 

build:
	cargo build --release --target x86_64-unknown-linux-musl

test-docker:
	docker-compose -f tests/docker-compose.yml up -d
	docker-compose -f tests/docker-compose.yml exec rust cargo test