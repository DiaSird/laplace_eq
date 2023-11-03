LOG_LEVEL?=debug
default: run

run:
	cargo run 

# powershell
log:
	@echo "develop mode: log level(${LOG_LEVEL})"
	@echo RUST_BACKTRACE=1;RUST_LOG=${LOG_LEVEL} && cargo run

.PHONY: log run