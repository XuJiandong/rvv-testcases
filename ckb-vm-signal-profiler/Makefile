

build:
	cargo build --release
	
profile-testcases: ../cases/target/riscv64imac-unknown-none-elf/release/rvv-testcases
	target/release/ckb-vm-signal-profiler $<
	pprof --top $< simple.profile
