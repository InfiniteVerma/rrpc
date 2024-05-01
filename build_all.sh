root=$PWD
cd client/ && cargo build
cd $root
cd server/ && cargo build
cd $root
cd shared/ && cargo build
