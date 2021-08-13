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

ci: clean
	docker build -t lib-rv32-test .
	docker run -it --rm lib-rv32-test .
	docker rmi lib-rv32-test
