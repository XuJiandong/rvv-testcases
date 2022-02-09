# rvv-testcases
Test cases of RISC-V V extension implementation on ckb-vm: https://github.com/nervosnetwork/ckb-vm/tree/rvv

### Prepare tools

Install ckb-debugger for RVV:
```bash
make install-tools
```

### Build

```bash
make all-via-docker
```

### Run
```bash
make run
```

### Debug mode
By default, it's in release mode. Turn it into debug mode:

```bash
export BUILD=debug
make all-via-docker
make run
```
### Run part of test cases

```bash
make run args=vop_vv_cases::test_vop_vv
```
Then only test case `vop_vv_cases::test_vop_vv` will be run. See macro `misc::test_case`.
