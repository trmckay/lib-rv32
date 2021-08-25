SHELL = bash
RUST_FILES = $(shell find . -type f -name '*.rs')

build: $(RUST_FILES)
	cargo build --verbose

test: $(RUST_FILES)
	cargo test --verbose

release: $(RUST_FILES)
	cargo build --verbose --release

clean:
	(cd isa-sim && cargo clean)
	(cd mcu && cargo clean)
	(cd assembler && cargo clean)
	(cd wasm && cargo clean)
	(cd common && cargo clean)
	rm -f mcu/programs/**/{*.elf,*.bin,dump.txt}
	rm -rf wasm/pkg

format: $(RUST_FILES)
	rustfmt **/*.rs

wasm: $(RUST_FILES)
	(cd wasm && wasm-pack build)

check: $(RUST_FILES)
	rustfmt **/*.rs --check
	cargo check --release
	(cd isa-sim && cargo check --release --target wasm32-unknown-unknown)
	(cd mcu && cargo check --release --target wasm32-unknown-unknown)
	(cd assembler && cargo check --release --target wasm32-unknown-unknown)

ci: clean
	docker build -t lib-rv32-test .
	docker run -it --rm lib-rv32-test .
	docker rmi lib-rv32-test
