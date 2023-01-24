.PHONY: watch test coverage lint format sec

watch:
	@cargo watch -x check

test:
	@cargo test

coverage:
	@cargo tarpaulin --ignore-tests

lint:
	@cargo clippy -- -D warnings
	@cargo fmt -- --check

format:
	@cargo fmt

sec:
	@cargo audit
