
CKB_DEBUGGER ?= ckb-debugger-rvv

all-via-docker:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code thewawar/ckb-capsule:2021-08-16 bash -c 'make all'

all:
	cargo build --bin rvv-testcases

run:
	RUST_LOG=debug ${CKB_DEBUGGER} --max-cycles 1000000000 --bin ./target/riscv64imac-unknown-none-elf/debug/rvv-testcases

expand:
	cargo expand --bin rvv-testcases --color never

disasm:
	riscv64-unknown-elf-objdump -d target/riscv64imac-unknown-none-elf/debug/rvv-testcases