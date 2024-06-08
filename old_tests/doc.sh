root=$PWD
cd client/ && cargo doc
cd $root
cd server/ && cargo doc
cd $root
cd shared/ && cargo doc
