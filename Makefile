SHELL = bash

build:
	cargo build --verbose

test:
	cargo test --verbose

release:
	cargo build --verbose --release

clean:
	(cd isa-sim && cargo clean)
	(cd mcu && cargo clean)
	(cd assembler && cargo clean)
	(cd wasm && cargo clean)
	(cd common && cargo clean)
	(cd wasm && cargo clean)
	(cd wasm/node && rm -rf dist)
	rm -rf wasm/pkg
	shopt -s globstar && rm -f mcu/programs/**/{*.elf,*.bin,dump.txt}

wasm:
	(cd wasm && wasm-pack build)

app:
	(cd wasm && wasm-pack build)
	(cd wasm/node && npm install -q && npm run-script build)
	(cd wasm/node && tar czvf riscv-wasm-app.tar.gz dist && rm -rf dist)
	mv wasm/node/riscv-wasm-app.tar.gz .

format:
	shopt -s globstar && rustfmt **/*.rs

check:
	shopt -s globstar && rustfmt **/*.rs --check
	cargo check --release
	(cd isa-sim && cargo check --release --target wasm32-unknown-unknown)
	(cd mcu && cargo check --release --target wasm32-unknown-unknown)
	(cd assembler && cargo check --release --target wasm32-unknown-unknown)
	(cd wasm && cargo check --release --target wasm32-unknown-unknown)

ci: clean
	docker build -t lib-rv32-test .
	docker run -it --rm lib-rv32-test .
	docker rmi lib-rv32-test

