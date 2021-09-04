SHELL = bash

build:
	cargo build --verbose

test:
	env RUST_BACKTRACE=1 cargo test --verbose

release:
	cargo build --verbose --release

check:
	shopt -s globstar && rustfmt **/*.rs --check
	cargo check --release
	(cd isa-sim && cargo check --release --target wasm32-unknown-unknown)
	(cd mcu && cargo check --release --target wasm32-unknown-unknown)
	(cd assembler && cargo check --release --target wasm32-unknown-unknown)
	(cd web-app && cargo check --release --target wasm32-unknown-unknown)

clean:
	(cd isa-sim && cargo clean)
	(cd mcu && cargo clean)
	(cd assembler && cargo clean)
	(cd web-app && cargo clean)
	(cd common && cargo clean)
	(cd web-app && cargo clean)
	(cd web-app/node && rm -rf dist)
	rm -rf web-app/pkg
	rm -rf web-app/node/node_modules
	shopt -s globstar && rm -f mcu/programs/**/{*.elf,*.bin,dump.txt}

app:
	(cd web-app && wasm-pack build)
	(cd web-app/node && npm install -q && npm run-script build)

serve: app
	( \
	    cd web-app && \
	    docker-compose down && \
	    docker-compose up -d && \
	    docker-compose ps \
	)

dist: app
	(cd web-app/node && tar czvf dist.tar.gz dist)
	mv web-app/node/dist.tar.gz .

format:
	shopt -s globstar && rustfmt **/*.rs

ci: clean
	docker build -t lib-rv32-test .
	docker run -it --rm lib-rv32-test .
	docker rmi lib-rv32-test
