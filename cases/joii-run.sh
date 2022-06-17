export RUST_LOG=debug
export RUST_BACKTRACE=1

export BUILD=debug
export BUILD=release

rvv_test_case_bin=$HOME/code/rvv-testcases/cases/target/riscv64imac-unknown-none-elf/$BUILD/rvv-testcases

clear

~/code/my_tools/expand_macro.py
if [ $? -ne 0 ]; then
    echo -e "\033[31mBuild rvv-testcase Failed!!!\033[0m"
    exit 1
fi

make all
if [ $? -ne 0 ]; then
    echo -e "\033[31mBuild rvv-testcase Failed!!!\033[0m"
    exit 1
fi
#clear
echo "make all success"

Run_Bin="int64"
Run_Bin="asm64"
#Run_Bin="ckb-debugger"

#case_name=test_vmsop_vv

#rvv_testargs_verbose=" --verbose"
#rvv_testargs_simple=" --simple"
rvv_testargs_seed=" --seed=22149"

rvv_test_args="--case="$case_name$rvv_testargs_verbose$rvv_testargs_simple$rvv_testargs_seed
echo $rvv_test_args

if [ $Run_Bin == "int64" ]; then
    echo "run in int64:"
    cd $HOME/code/ckb-vm
    cargo build --example int64
    if [ $? -ne 0 ]; then
        echo -e "\033[31mBuild int64 Failed!!!\033[0m"
        exit 1
    fi
    echo "make ckb-vm int64 success"

    ckb_vm_bin=$HOME/code/ckb-vm/target/debug/examples/int64

    $ckb_vm_bin $rvv_test_case_bin $rvv_test_args
    if [ $? -ne 0 ]; then
        echo -e "\033[31mRun Testcase Failed!!!\033[0m"
    else
        echo "Run Testcase Success!!!"
    fi

elif [ $Run_Bin == "asm64" ]; then
    echo "run in asm64:"
    cd $HOME/code/ckb-vm
    cargo build --example asm64 --features asm
    if [ $? -ne 0 ]; then
        echo -e "\033[31mBuild asm64 Failed!!!\033[0m"
        exit 1
    fi
    echo "make ckb-vmand asm64 success"

    ckb_vm_bin=$HOME/code/ckb-vm/target/debug/examples/asm64

    $ckb_vm_bin $rvv_test_case_bin $rvv_test_args
    if [ $? -ne 0 ]; then
        echo -e "\033[31mRun Testcase Failed!!!\033[0m"
    else
        echo -e "Run Testcase Success!!!"
    fi

elif [ $Run_Bin == "ckb-debugger" ]; then
    echo "run in ckb-debugger:"

    cd $HOME/code/ckb-standalone-debugger/bins/
    cargo build
    if [ $? -ne 0 ]; then
        echo -e "\033[Build ckb-debugger Failed!!!\033[0m"
        exit 1
    fi

    ckb_debuger_bin=$HOME/code/ckb-standalone-debugger/bins/target/debug/ckb-debugger-rvv
    $ckb_debuger_bin --max-cycles 1000000000 --bin $rvv_test_case_bin -- $case_name
    if [ $? -ne 0 ]; then
        echo -e "\033[31mRun Testcase Failed!!!\033[0m"
    else
        echo "Run Testcase Success!!!"
    fi

else
    echo "unknow type"
    exit 1
fi

