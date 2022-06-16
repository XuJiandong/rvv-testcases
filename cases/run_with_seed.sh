
CUR_PWD=$(pwd)
CUR_DIR=$(cd `dirname $0`; pwd)

cd $CUR_DIR/../tools/gen_rand/
cargo run -- $RANDOM
#cargo run

cd $CUR_DIR/
make install-tool2
make all-via-docker
make asm64-run args='--seed=$RANDOM'
