root=$PWD
cd client/ && cargo clean
cd $root
cd server/ && cargo clean
cd $root
cd shared/ && cargo clean
