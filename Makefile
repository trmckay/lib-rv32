SHELL = bash

build:
	cargo build --verbose

test:
	cargo test --verbose

releae:
	cargo build --verbose --release

clean:
	rm -rf **/target
	rm -f mcu/programs/**/{*.elf,*.bin,dump.txt}

format:
	rustfmt **/*.rs

check:
	rustfmt **/*.rs --check
	cargo check --release
	(cd common && cargo check --release --target wasm32-unknown-unknown)
	(cd isa-sim && cargo check --release --target wasm32-unknown-unknown)
	(cd mcu && cargo check --release --target wasm32-unknown-unknown)
	(cd assembler && cargo check --release --target wasm32-unknown-unknown)

ci: clean
	docker build -t lib-rv32-test .
	docker run -it --rm lib-rv32-test .
	docker rmi lib-rv32-test
