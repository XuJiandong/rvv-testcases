
CKB_DEBUGGER ?= ckb-debugger-rvv
BUILD ?= debug
PORT := 9999
BUILDER_DOCKER := thewawar/ckb-capsule:2021-12-25

all-via-docker:
	docker run --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${BUILDER_DOCKER} bash -c 'make all'

shell:
	docker run -it --rm -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${BUILDER_DOCKER} bash

all:
	if test "${BUILD}" = "release"; then\
		cargo build --bin rvv-testcases --release;\
	else\
		cargo build --bin rvv-testcases;\
	fi

run:
	RUST_LOG=debug ${CKB_DEBUGGER} --max-cycles 1000000000 --bin ./target/riscv64imac-unknown-none-elf/${BUILD}/rvv-testcases

expand:
	cargo expand --bin rvv-testcases --color never

disasm:
	riscv64-unknown-elf-objdump -d target/riscv64imac-unknown-none-elf/${BUILD}/rvv-testcases | rustfilt

start-gdb-server:
	${CKB_DEBUGGER} --mode gdb --gdb-listen 127.0.0.1:${PORT} --bin ./target/riscv64imac-unknown-none-elf/${BUILD}/rvv-testcases

start-docker:
	docker run --rm -it -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${BUILDER_DOCKER} bash

run-gdb-rust:
	riscv64-unknown-elf-gdb -ex "target remote host.docker.internal:${PORT}" ./target/riscv64imac-unknown-none-elf/${BUILD}/rvv-testcases

install-tools:
	cargo install --branch rvv --git https://github.com/mohanson/ckb-standalone-debugger.git
