
CKB_DEBUGGER ?= ckb-debugger-rvv
BUILD ?= release
PORT := 9999
BUILDER_DOCKER := thewawar/ckb-capsule:2021-12-25
DOCKER_ARGS = --rm -e BUILD=$(BUILD) -eOWNER=`id -u`:`id -g` -v `pwd`:/code -v ${HOME}/.cargo/git:/root/.cargo/git -v ${HOME}/.cargo/registry:/root/.cargo/registry -w/code ${BUILDER_DOCKER}

all-via-docker:
	docker run $(DOCKER_ARGS) bash -c 'make all'

check-via-docker:
	docker run $(DOCKER_ARGS) bash -c 'cargo check'

fix-via-docker:
	docker run $(DOCKER_ARGS) bash -c 'cargo fix --allow-no-vcs'

shell:
	docker run --rm -it $(DOCKER_ARGS) bash

all:
	if test "${BUILD}" = "release"; then\
		cargo build --bin rvv-testcases --release;\
	else\
		cargo build --bin rvv-testcases;\
	fi
# for example:
# > make run 
# run all test cases
#
# > make run args=test_vop_vv`
# to run one test case including 'test_vop_vv'
#
# > make run args='"" verbose'
# to run all test cases with verbose
run:
	RUST_LOG=debug ${CKB_DEBUGGER} --max-cycles 1000000000 --bin ./target/riscv64imac-unknown-none-elf/${BUILD}/rvv-testcases -- $(args)


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
