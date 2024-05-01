root=$PWD
cd client/ && cargo fmt
cd $root
cd server/ && cargo fmt
cd $root
cd shared/ && cargo fmt
