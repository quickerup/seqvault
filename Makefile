.PHONY: build test run-api deploy-contracts

build:
	cargo build --workspace

test:
	cargo test --workspace
	cd contracts && npm test

run-api:
	cargo run -p api

deploy-contracts:
	cd contracts && npm run deploy
